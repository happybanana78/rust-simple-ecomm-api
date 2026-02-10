use crate::auth::dto::{AuthScopes, AuthToken, AuthUserId};
use crate::auth::traits::Scope;
use crate::responses::error_responses::ErrorResponse;
use crate::state::AppState;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct AuthMiddleware {
    permission_scope: Option<Arc<dyn Scope + Send + Sync>>,
}

impl AuthMiddleware {
    pub fn new(permission_scope: Option<Arc<dyn Scope + Send + Sync>>) -> Self {
        Self { permission_scope }
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
            permission_scope: self.permission_scope.clone(),
        })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
    permission_scope: Option<Arc<dyn Scope + Send + Sync>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
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
        let state = req
            .app_data::<Data<AppState>>()
            .expect("App State missing from app data")
            .get_ref();

        let auth_service = state.auth_service.clone();

        let required_scope = self.permission_scope.as_ref().map(|scope| scope.as_str());

        Box::pin(async move {
            let token = match extract_bearer_token(req.request()) {
                Some(t) => t,
                None => {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(ErrorResponse::new("missing bearer".to_string()))
                            .map_into_left_body(),
                    ));
                }
            };

            let Some(auth_token_model) = auth_service.get_token_if_exist(token).await? else {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ErrorResponse::new("invalid token".to_string()))
                        .map_into_left_body(),
                ));
            };

            let auth_token = AuthToken::from(auth_token_model);

            if auth_token.is_expired() {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ErrorResponse::new("expired token".to_string()))
                        .map_into_left_body(),
                ));
            }

            if required_scope.is_some_and(|scope| !auth_token.scopes.contains(scope)) {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ErrorResponse::new("permission not sufficient".to_string()))
                        .map_into_left_body(),
                ));
            }

            req.extensions_mut().insert(AuthUserId(auth_token.user_id));
            req.extensions_mut().insert(AuthScopes(auth_token.scopes));

            service.call(req).await.map(|res| res.map_into_right_body())
        })
    }
}

fn extract_bearer_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}
