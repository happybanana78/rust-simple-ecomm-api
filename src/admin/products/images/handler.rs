use crate::admin::products::images::dto::{
    CreateProductImageCommand, CreateProductImageDTO, UpdateProductImageSortCommand,
    UpdateProductImageSortDTO,
};
use crate::errors::error::AppError;
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};
use tokio::fs;

pub async fn upload(
    state: web::Data<AppState>,
    form: MultipartForm<CreateProductImageDTO>,
) -> Result<impl Responder, AppError> {
    let temp_file = &form.file;

    let bytes = fs::read(temp_file.file.path())
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let extension = temp_file
        .content_type
        .as_ref()
        .ok_or_else(|| AppError::Internal("invalid mime".to_string()))?
        .subtype()
        .to_string();

    let command = CreateProductImageCommand::new_from_dto(&form.into_inner());

    state
        .admin_product_images_service
        .upload(command, &state.local_storage, bytes, extension.as_str())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn update_sort(
    state: web::Data<AppState>,
    body: web::Json<UpdateProductImageSortDTO>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let command = UpdateProductImageSortCommand::new_from_dto(&body.into_inner())?;

    let new_sort = state
        .admin_product_images_service
        .update_sort(id.into_inner(), command)
        .await?;

    Ok(HttpResponse::Ok().json(SuccessResponse::ok(new_sort)))
}

pub async fn delete(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    state
        .admin_product_images_service
        .delete(id.into_inner(), &state.local_storage)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
