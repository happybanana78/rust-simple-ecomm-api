use crate::cart::cart_items::dto::{
    AddItemCommand, AddItemDto, RemoveItemCommand, RemoveItemDto, UpdateItemCommand, UpdateItemDto,
};
use crate::cart::cart_items::service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;
use validator::Validate;

#[get("/add")]
pub async fn add_item(
    pool: web::Data<PgPool>,
    body: web::Json<AddItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = AddItemCommand::try_from(body.into_inner())?;
    let items = service::add_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(items)))
}

#[get("/remove")]
pub async fn remove_item(
    pool: web::Data<PgPool>,
    body: web::Json<RemoveItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = RemoveItemCommand::try_from(body.into_inner())?;
    service::remove_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}

#[get("/update")]
pub async fn update_item(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = UpdateItemCommand::try_from(body.into_inner())?;
    service::update_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}
