use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;
use crate::cart::dto::{GetCartCommand, GetCartDto};
use crate::cart::service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;

#[post("/get")]
pub async fn get_user_cart(
    pool: web::Data<PgPool>,
    body: web::Path<GetCartDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = body.into_inner().into_command();
    let cart = match command {
        GetCartCommand::ByUser(cmd) => {
            service::get_cart_by_user(&pool, cmd.user_id).await?
        }
        GetCartCommand::ByHash(cmd) => {
            service::get_cart_by_hash(&pool, cmd.user_hash).await?
        }
    };

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(cart)))
}
