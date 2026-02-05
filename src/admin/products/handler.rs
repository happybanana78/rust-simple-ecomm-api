use super::service;
use crate::admin::products::dto::{
    CreateProductCommand, CreateProductDTO, UpdateProductCommand, UpdateProductDTO,
};
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;
use validator::Validate;

pub async fn index(pool: web::Data<PgPool>) -> Result<impl Responder, AppError> {
    let products = service::index(&pool).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(products)))
}

pub async fn show(pool: web::Data<PgPool>, id: web::Path<i64>) -> Result<impl Responder, AppError> {
    let product = service::show(&pool, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(product)))
}

pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateProductDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = CreateProductCommand::try_from(body.into_inner())?;
    let products = service::create(&pool, command).await?;
    Ok(HttpResponse::Created().json(products))
}

pub async fn update(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateProductDTO>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = UpdateProductCommand::try_from(body.into_inner())?;
    service::update(&pool, command, id.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete(
    pool: web::Data<PgPool>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    service::delete(&pool, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
