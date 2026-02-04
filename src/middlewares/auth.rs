use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web::body::{BoxBody, EitherBody};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use sqlx::PgPool;
use crate::auth::dto::AuthToken;
use crate::auth::repository;

pub struct AuthMiddleware {
    pool: PgPool,
}

impl AuthMiddleware {
    pub fn new(pool: PgPool) -> Self {
        Self { pool}
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner {
            service: Rc::new(service),
            pool: self.pool.clone(),
        })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
    pool: PgPool,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error
    > + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let pool = self.pool.clone();

        Box::pin(async move {
            let token = match extract_bearer_token(req.request()) {
                Some(t) => t,
                None => {
                    return Ok(
                        req.into_response(
                            HttpResponse::Unauthorized().finish().map_into_left_body()
                        )
                    );
                }
            };

            let auth_token_model = repository::get_token(&pool, token).await?;

            let auth_token = AuthToken::from(auth_token_model);

            if auth_token.is_expired() {
                return Ok(
                    req.into_response(
                        HttpResponse::Unauthorized().finish().map_into_left_body()
                    )
                );
            }

            req.extensions_mut().insert(auth_token.user_id);
            req.extensions_mut().insert(auth_token.scopes);

            service.call(req).await
                .map(|res| res.map_into_right_body())
        })
    }
}

pub fn extract_bearer_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}
