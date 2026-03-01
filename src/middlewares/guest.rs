use crate::app::users::dto::{GuestDto, GuestToken};
use crate::responses::error_responses::ErrorResponse;
use crate::state::AppState;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct GuestMiddleware;

impl<S, B> Transform<S, ServiceRequest> for GuestMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Transform = GuestMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(GuestMiddlewareInner {
            service: Rc::new(service),
        })
    }
}

pub struct GuestMiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for GuestMiddlewareInner<S>
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

        let user_service = state.user_service.clone();

        Box::pin(async move {
            let user_hash = match extract_hash_token(req.request()) {
                Some(t) => t,
                None => {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(ErrorResponse::new("missing guest token".to_string()))
                            .map_into_left_body(),
                    ));
                }
            };

            let Some(user_hash_model) = user_service.get_user_hash(user_hash.as_str()).await?
            else {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ErrorResponse::new("token not found".to_string()))
                        .map_into_left_body(),
                ));
            };

            let guest = GuestDto::from(user_hash_model);

            if guest.is_expired() {
                return Ok(
                    req.into_response(HttpResponse::Unauthorized().finish().map_into_left_body())
                );
            }

            req.extensions_mut().insert(GuestToken(guest.hash));

            service.call(req).await.map(|res| res.map_into_right_body())
        })
    }
}

fn extract_hash_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("x-guest-token")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}
