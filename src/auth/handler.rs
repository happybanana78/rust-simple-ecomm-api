use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use validator::Validate;
use crate::auth::dto::{LoginCommand, LoginDTO, PublicUser, RegisterCommand, RegisterDTO};
use crate::auth::service;
use crate::errors::error::AppError;

#[post("/auth/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterDTO>
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = RegisterCommand::try_from(body.into_inner())?;
    let user = service::register(&pool, command).await?;
    Ok(HttpResponse::Created().json(PublicUser::from(user)))
}

#[post("/auth/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginDTO>
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let command = LoginCommand::try_from(body.into_inner())?;
    let auth_token = service::login(&pool, command).await?;
    Ok(HttpResponse::Ok().json(auth_token))
}
