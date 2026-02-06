use crate::auth::dto::AuthUserId;
use crate::cart::dto::{GetCartCommandEnum, GetCartDto};
use crate::cart::service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;
use validator::Validate;

pub async fn get_user_cart(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<GetCartDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    if let Some(dto_user_id) = body.user_id {
        let auth_user_id = request
            .extensions()
            .get::<AuthUserId>()
            .map(|id| id.0)
            .ok_or(AppError::Unauthorized("unauthorized".to_string()))?;

        if dto_user_id != auth_user_id {
            return Err(AppError::Forbidden("ownership_required".into()));
        }
    }

    let command = body.into_inner().into_command();

    match command {
        GetCartCommandEnum::ByUser(cmd) => Ok(HttpResponse::Ok().json(SuccessResponse::ok(
            service::get_cart_by_user(&pool, cmd.user_id).await?,
        ))),
        GetCartCommandEnum::ByHash(cmd) => Ok(HttpResponse::Ok().json(SuccessResponse::ok(
            service::get_cart_by_hash(&pool, cmd.user_hash).await?,
        ))),
    }
}
