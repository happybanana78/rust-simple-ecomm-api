use crate::app::cart::cart_items::dto::{
    AddItemCommand, AddItemDto, RemoveItemCommand, RemoveItemDto, UpdateItemCommand, UpdateItemDto,
};
use crate::errors::error::AppError;
use crate::extractors::extract_auth_user_id;
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use validator::Validate;

pub async fn get_user_cart(
    request: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let auth_user_id = extract_auth_user_id(&request)?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(
        state
            .user_cart_service
            .get_cart_by_user(&auth_user_id)
            .await?,
    )))
}

pub async fn add_item(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<AddItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = extract_auth_user_id(&request)?;

    let cart_id = state
        .user_cart_service
        .get_cart_id_by_user(&auth_user_id)
        .await?;

    let command = AddItemCommand::new(body.into_inner(), cart_id);

    let product = state.product_service.exist(command.product_id).await?;
    if !product {
        return Err(AppError::NotFound("product not found".to_string()));
    }

    state.cart_items_service.add_item(command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(())))
}

pub async fn update_item(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<UpdateItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = extract_auth_user_id(&request)?;

    let cart_id = state
        .user_cart_service
        .get_cart_id_by_user(&auth_user_id)
        .await?;

    let command = UpdateItemCommand::new(body.into_inner(), cart_id);

    let product = state.product_service.exist(command.product_id).await?;
    if !product {
        return Err(AppError::NotFound("product not found".to_string()));
    }

    state.cart_items_service.update_item(command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}

pub async fn remove_item(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<RemoveItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = extract_auth_user_id(&request)?;

    let cart_id = state
        .user_cart_service
        .get_cart_id_by_user(&auth_user_id)
        .await?;

    let command = RemoveItemCommand::new(body.into_inner(), cart_id);

    let product = state.product_service.exist(command.product_id).await?;
    if !product {
        return Err(AppError::NotFound("product not found".to_string()));
    }

    state.cart_items_service.remove_item(command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}
