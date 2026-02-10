use crate::errors::error::AppError;
use crate::products::dto::PublicProduct;
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn index(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let products = state.product_service.get_all_public().await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(products)))
}

pub async fn show(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let product = state
        .product_service
        .get_one_public(&id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(PublicProduct::from(product))))
}
