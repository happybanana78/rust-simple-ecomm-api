use crate::admin::reviews::dto::{
    IndexReviewDTO, UpdateReviewStatusCommand, UpdateReviewStatusDTO,
};
use crate::admin::reviews::filters::AdminReviewFilters;
use crate::errors::error::AppError;
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use crate::utils::pagination::Paginate;
use actix_web::{HttpResponse, Responder, web};
use validator::Validate;

pub async fn index(
    state: web::Data<AppState>,
    body: web::Query<IndexReviewDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let pagination = Paginate::new(body.limit.unwrap(), body.page.unwrap());

    let filters = AdminReviewFilters::try_from(body.clone().into_inner())?;

    let reviews = state
        .admin_reviews_service
        .get_all_paginated_public(&pagination, &filters, &body.search)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok_with_pagination(
        reviews.data,
        pagination,
    )))
}

pub async fn show(
    state: web::Data<AppState>,
    review_id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let review = state
        .admin_reviews_service
        .get_one_public(review_id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(review)))
}

pub async fn update_status(
    state: web::Data<AppState>,
    body: web::Json<UpdateReviewStatusDTO>,
    review_id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = UpdateReviewStatusCommand::try_from(body.into_inner())?;

    state
        .admin_reviews_service
        .update_status(command, review_id.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete(
    state: web::Data<AppState>,
    review_id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    state
        .admin_reviews_service
        .delete(review_id.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
