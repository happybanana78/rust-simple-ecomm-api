use crate::admin::products::images::dto::CreateProductImageDTO;
use crate::errors::error::AppError;
use crate::state::AppState;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};

pub async fn upload(
    state: web::Data<AppState>,
    form: MultipartForm<CreateProductImageDTO>,
) -> Result<impl Responder, AppError> {
    state
        .admin_product_images_service
        .upload(form.into_inner(), &state.local_storage)
        .await?;

    Ok(HttpResponse::NoContent().finish())
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
