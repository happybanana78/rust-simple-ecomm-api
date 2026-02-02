use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use super::service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use crate::products::dto::PublicProduct;

#[get("")]
pub async fn index(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let products = service::index(&pool).await?;

    let public_products: Vec<PublicProduct> = products.into_iter().map(PublicProduct::from).collect();

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(public_products)))
}

#[get("/{id}")]
pub async fn show(
    pool: web::Data<PgPool>,
    id: web::Path<i64>
) -> Result<impl Responder, AppError> {
    let product = service::show(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(PublicProduct::from(product))))
}
