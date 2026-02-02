use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use super::service;
use validator::Validate;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use crate::products::dto::{CreateProductCommand, CreateProductDTO, UpdateProductCommand, UpdateProductDTO};

#[get("")]
pub async fn index(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let products = service::index(&pool).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(products)))
}

#[get("/{id}")]
pub async fn show(
    pool: web::Data<PgPool>,
    id: web::Path<i64>
) -> Result<impl Responder, AppError> {
    let product = service::show(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(product)))
}

#[post("")]
pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateProductDTO>
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = CreateProductCommand::try_from(body.into_inner())?;
    let products = service::create(&pool, command).await?;
    Ok(HttpResponse::Created().json(products))
}

#[put("/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateProductDTO>,
    id: web::Path<i64>
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = UpdateProductCommand::try_from(body.into_inner())?;
    service::update(&pool, command, id.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    id: web::Path<i64>
) -> Result<impl Responder, AppError> {
    service::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
