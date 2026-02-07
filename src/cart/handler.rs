use crate::auth::dto::AuthUserId;
use crate::cart::dto::{GetGuestCartDto, GetUserCartDto};
use crate::cart::service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;
use validator::Validate;

pub async fn get_user_cart(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<GetUserCartDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = request
        .extensions()
        .get::<AuthUserId>()
        .map(|id| id.0)
        .ok_or(AppError::Unauthorized("unauthorized".to_string()))?;

    if body.user_id != Some(auth_user_id) {
        return Err(AppError::Forbidden("ownership_required".into()));
    }

    let command = body.into_inner().into_command();

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(
        service::get_cart_by_user(&pool, command.user_id).await?,
    )))
}

pub async fn get_guest_cart(
    pool: web::Data<PgPool>,
    body: web::Json<GetGuestCartDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = body.into_inner().into_command();

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(
        service::get_cart_by_hash(&pool, command.user_hash).await?,
    )))
}
