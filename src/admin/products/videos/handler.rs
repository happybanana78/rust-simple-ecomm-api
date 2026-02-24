use crate::admin::products::videos::dto::CreateProductVideoDTO;
use crate::errors::error::AppError;
use crate::state::AppState;
use actix_files::{Files, NamedFile};
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, web};

pub async fn upload(
    state: web::Data<AppState>,
    form: MultipartForm<CreateProductVideoDTO>,
) -> Result<impl Responder, AppError> {
    state
        .admin_product_videos_service
        .upload(form.into_inner(), &state.local_storage)
        .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn stream(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    let video_file = state
        .admin_product_videos_service
        .stream(id.into_inner())
        .await?;

    Ok(video_file)
}

pub async fn delete(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, AppError> {
    state
        .admin_product_videos_service
        .delete(id.into_inner(), &state.local_storage)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

// TODO: implement sort update
