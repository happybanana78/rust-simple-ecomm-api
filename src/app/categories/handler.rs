use crate::admin::categories::dto::IndexCategoryDTO;
use crate::errors::error::AppError;
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use crate::utils::pagination::Paginate;
use actix_web::{HttpResponse, Responder, web};
use validator::Validate;

pub async fn index(
    state: web::Data<AppState>,
    body: web::Query<IndexCategoryDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let pagination = Paginate::new(body.limit.unwrap(), body.page.unwrap());

    let categories = state
        .category_service
        .get_all_paginated_public(&pagination, &body.search)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok_with_pagination(
        categories.data,
        pagination,
    )))
}

pub async fn show(
    state: web::Data<AppState>,
    slug: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let category = state
        .category_service
        .get_one_public(&slug.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(category)))
}
