use crate::cart::cart_items::dto::{
    AddItemCommand, AddItemDto, RemoveItemCommand, RemoveItemDto, UpdateItemCommand, UpdateItemDto,
};
use crate::errors::error::AppError;
use crate::errors::response::SuccessResponse;
use crate::state::AppState;
use crate::users::dto::GuestToken;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use validator::Validate;

pub async fn get_guest_cart(
    request: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let guest_token = extract_guest_token(&request)?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(
        state
            .guest_cart_service
            .get_cart_by_hash(&guest_token)
            .await?,
    )))
}

pub async fn add_item(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<AddItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let guest_token = extract_guest_token(&request)?;

    let cart_id = state
        .guest_cart_service
        .get_cart_id_by_hash(&guest_token)
        .await?;

    let command = AddItemCommand::new(body.into_inner(), cart_id);

    state.cart_items_service.add_item(command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::ok(())))
}

pub async fn update_item(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<UpdateItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let guest_token = extract_guest_token(&request)?;

    let cart_id = state
        .guest_cart_service
        .get_cart_id_by_hash(&guest_token)
        .await?;

    let command = UpdateItemCommand::new(body.into_inner(), cart_id);

    state.cart_items_service.update_item(command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}

pub async fn remove_item(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<RemoveItemDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let guest_token = extract_guest_token(&request)?;

    let cart_id = state
        .guest_cart_service
        .get_cart_id_by_hash(&guest_token)
        .await?;

    let command = RemoveItemCommand::new(body.into_inner(), cart_id);

    state.cart_items_service.remove_item(command).await?;
    Ok(HttpResponse::Ok().json(SuccessResponse::<()>::empty()))
}

fn extract_guest_token(req: &HttpRequest) -> Result<String, AppError> {
    req.extensions()
        .get::<GuestToken>()
        .map(|s| s.0.clone())
        .ok_or(AppError::Unauthorized("guest token missing".to_string()))
}
