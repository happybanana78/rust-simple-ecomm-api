use crate::auth::dto::{LoginCommand, LoginDTO, PublicUser, RegisterCommand, RegisterDTO};
use crate::errors::error::AppError;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder, post, web};
use validator::Validate;

#[post("/auth/register")]
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = RegisterCommand::try_from(body.into_inner())?;
    let user = state.auth_service.register(command).await?;
    Ok(HttpResponse::Created().json(PublicUser::from(user)))
}

#[post("/auth/login")]
pub async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginDTO>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = LoginCommand::try_from(body.into_inner())?;
    let auth_token = state.auth_service.login(command).await?;
    Ok(HttpResponse::Ok().json(auth_token))
}
