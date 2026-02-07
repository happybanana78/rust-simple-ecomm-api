use crate::cart::cart_items::dto::{
    AddItemCommand, AddItemDto, RemoveItemCommand, RemoveItemDto, UpdateItemCommand, UpdateItemDto,
};
use crate::cart::cart_items::service as cart_items_service;
use crate::cart::guest_cart::service as cart_service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use crate::users::dto::GuestToken;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;
use validator::Validate;

pub async fn get_guest_cart(
    request: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {
    let guest_token = request
        .extensions()
        .get::<GuestToken>()
        .map(|s| s.0.clone())
        .ok_or(AppError::Unauthorized("guest token missing".to_string()))?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(
        cart_service::get_cart_by_hash(&pool, &guest_token).await?,
    )))
}

pub async fn add_item(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<AddItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let guest_token = request
        .extensions()
        .get::<GuestToken>()
        .cloned()
        .ok_or(AppError::Unauthorized("guest token missing".to_string()))?;

    let cart_id = guest_token.get_cart_id(&pool).await?;

    let command = AddItemCommand::new(body.into_inner(), cart_id);

    cart_items_service::add_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(())))
}

pub async fn update_item(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<UpdateItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let guest_token = request
        .extensions()
        .get::<GuestToken>()
        .cloned()
        .ok_or(AppError::Unauthorized("guest token missing".to_string()))?;

    let cart_id = guest_token.get_cart_id(&pool).await?;

    let command = UpdateItemCommand::new(body.into_inner(), cart_id);

    cart_items_service::update_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}

pub async fn remove_item(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<RemoveItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let guest_token = request
        .extensions()
        .get::<GuestToken>()
        .cloned()
        .ok_or(AppError::Unauthorized("guest token missing".to_string()))?;

    let cart_id = guest_token.get_cart_id(&pool).await?;

    let command = RemoveItemCommand::new(body.into_inner(), cart_id);

    cart_items_service::remove_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}
