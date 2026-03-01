use crate::admin::categories::dto::{
    CreateCategoryCommand, CreateCategoryDTO, IndexCategoryDTO, UpdateCategoryCommand,
    UpdateCategoryDTO,
};
use crate::admin::categories::filters::CategoryFilters;
use crate::admin::categories::traits::IntoPublic;
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

    let filters = CategoryFilters::try_from(body.clone().into_inner())?;

    let categories = state
        .admin_category_service
        .get_all_paginated_public(&pagination, &filters, &body.search)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok_with_pagination(
        categories.data,
        pagination,
    )))
}

pub async fn show(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let category = state
        .admin_category_service
        .get_one_public(id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(category)))
}

pub async fn create(
    state: web::Data<AppState>,
    body: web::Json<CreateCategoryDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = CreateCategoryCommand::try_from(body.into_inner())?;
    let category = state.admin_category_service.create(command).await?;

    Ok(HttpResponse::Created().json(SuccessResponse::ok(category.into_public())))
}

pub async fn update(
    state: web::Data<AppState>,
    body: web::Json<UpdateCategoryDTO>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = UpdateCategoryCommand::try_from(body.into_inner())?;
    state
        .admin_category_service
        .update(command, id.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    state.admin_category_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
