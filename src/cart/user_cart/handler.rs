use crate::auth::dto::AuthUserId;
use crate::cart::cart_items::dto::{
    AddItemCommand, AddItemDto, RemoveItemCommand, RemoveItemDto, UpdateItemCommand, UpdateItemDto,
};
use crate::cart::cart_items::service as cart_items_service;
use crate::cart::user_cart::service as user_cart_service;
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use crate::products::service as product_service;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;
use validator::Validate;

pub async fn get_user_cart(
    request: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, AppError> {
    let auth_user_id = request
        .extensions()
        .get::<AuthUserId>()
        .map(|id| id.0)
        .ok_or(AppError::Unauthorized("unauthorized".to_string()))?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(
        user_cart_service::get_cart_by_user(&pool, &auth_user_id).await?,
    )))
}

pub async fn add_item(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<AddItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = request
        .extensions()
        .get::<AuthUserId>()
        .copied()
        .ok_or(AppError::Unauthorized("unauthorized".to_string()))?;

    let cart_id = auth_user_id.get_cart_id(&pool).await?;

    let command = AddItemCommand::new(body.into_inner(), cart_id);

    let product = product_service::check(&pool, &command.product_id).await?;
    if !product {
        return Err(AppError::NotFound("product not found".to_string()));
    }

    // TODO: handle code repetition (mostly in checks)

    cart_items_service::add_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(())))
}

pub async fn update_item(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<UpdateItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = request
        .extensions()
        .get::<AuthUserId>()
        .copied()
        .ok_or(AppError::Unauthorized("unauthorized".to_string()))?;

    let cart_id = auth_user_id.get_cart_id(&pool).await?;

    let command = UpdateItemCommand::new(body.into_inner(), cart_id);

    let product = product_service::check(&pool, &command.product_id).await?;
    if !product {
        return Err(AppError::NotFound("product not found".to_string()));
    }

    cart_items_service::update_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}

pub async fn remove_item(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<RemoveItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = request
        .extensions()
        .get::<AuthUserId>()
        .copied()
        .ok_or(AppError::Unauthorized("unauthorized".to_string()))?;

    let cart_id = auth_user_id.get_cart_id(&pool).await?;

    let command = RemoveItemCommand::new(body.into_inner(), cart_id);

    let product = product_service::check(&pool, &command.product_id).await?;
    if !product {
        return Err(AppError::NotFound("product not found".to_string()));
    }

    cart_items_service::remove_item(&pool, command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}
