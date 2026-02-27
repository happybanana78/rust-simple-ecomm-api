use crate::errors::error::AppError;
use crate::extractors::extract_auth_user_id;
use crate::products::reviews::dto::{CreateProductReviewCommand, CreateProductReviewDto};
use crate::responses::error_responses::SuccessResponse;
use crate::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use validator::Validate;

pub async fn create_user(
    request: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CreateProductReviewDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let auth_user_id = extract_auth_user_id(&request)?;

    let command = CreateProductReviewCommand::from_dto(body.into_inner(), Some(auth_user_id));
    let review = state.reviews_service.create(command).await?;

    Ok(HttpResponse::Created().json(SuccessResponse::ok(review)))
}

pub async fn create_guest(
    state: web::Data<AppState>,
    body: web::Json<CreateProductReviewDto>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = CreateProductReviewCommand::from_dto(body.into_inner(), None);
    let review = state.reviews_service.create(command).await?;

    Ok(HttpResponse::Created().json(SuccessResponse::ok(review)))
}
