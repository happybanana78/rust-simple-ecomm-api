use crate::admin::products::dto::{
    CreateProductCommand, CreateProductDTO, IndexProductDTO, UpdateProductCommand, UpdateProductDTO,
};
use crate::admin::products::filters::ProductFilters;
use crate::admin::products::traits::IntoPublic;
use crate::errors::error::AppError;
use crate::pagination::Paginate;
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
        .admin_product_service
        .get_all_paginated_public(&pagination, &filters, &body.search)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok_with_pagination(
        products.data,
        pagination,
    )))
}

pub async fn show(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let product = state
        .admin_product_service
        .get_one_public(id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(product)))
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateProductDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = CreateProductCommand::try_from(body.into_inner())?;
    let product = state.admin_product_service.create(command).await?;

    Ok(HttpResponse::Created().json(SuccessResponse::ok(product.into_public())))
}

pub async fn update(
    state: web::Data<AppState>,
    body: web::Json<UpdateProductDTO>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = UpdateProductCommand::try_from(body.into_inner())?;
    state
        .admin_product_service
        .update(command, id.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    state.admin_product_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

// TODO: add slug to product
