use crate::errors::error::AppError;
use crate::pagination::Paginate;
use crate::products::dto::IndexProductDTO;
use crate::products::filters::ProductFilters;
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, web};
use validator::Validate;

pub async fn index(
    state: web::Data<AppState>,
    body: web::Query<IndexProductDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let pagination = Paginate::new(body.limit.unwrap(), body.page.unwrap());

    let filters = ProductFilters::try_from(body.clone().into_inner())?;

    let products = state
        .product_service
        .get_all_paginated_public(&pagination, &filters, &body.search)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok_with_pagination(
        products.data,
        pagination,
    )))
}

pub async fn show(
    state: web::Data<AppState>,
    slug: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let product = state
        .product_service
        .get_one_public(slug.into_inner().as_str())
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(product)))
}
