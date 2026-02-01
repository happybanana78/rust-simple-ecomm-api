use super::repository;
use sqlx::PgPool;
use crate::auth::dto::{AuthToken, LoginCommand, NewUser, PublicAuthToken, RegisterCommand};
use crate::auth::model::UserModel;
use crate::errors::error::AppError;
use argon2::{password_hash::{PasswordHasher}, Argon2, PasswordVerifier};
use argon2::password_hash::phc::PasswordHash;

pub async fn register(pool: &PgPool, cmd: RegisterCommand) -> Result<UserModel, AppError>
{
    let hashed_password = hash_password(cmd.password.as_str())?;

    let new_user = NewUser {
        username: cmd.username,
        email: cmd.email,
        hashed_password,
    };

    let user = repository::find_by_email(pool, &new_user.email).await?;

    if user.is_some() {
        return Err(AppError::Conflict("user already exists".to_string()))
    }

    repository::register(pool, new_user).await
}

pub async fn login(pool: &PgPool, cmd: LoginCommand) -> Result<PublicAuthToken, AppError>
{
    let user = repository::find_by_email(pool, &cmd.email).await?
        .ok_or_else(|| AppError::Unauthorized("wrong credentials".to_string()))?;

    let valid_password = verify_password(&cmd.password, &user.password)?;

    if ! valid_password {
        return Err(AppError::Unauthorized("wrong credentials".to_string()))
    }

    let auth_token = get_auth_token(&pool, user.id).await?;

    Ok(PublicAuthToken::from(auth_token))
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes())
        .map_err(|_| AppError::Internal("failed to hash password".to_string()))?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::Internal("invalid password hash".into()))?;

    Ok(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    )
}

async fn get_auth_token(pool: &PgPool, user_id: i64) -> Result<AuthToken, AppError> {
    let check_token = repository::get_token_by_user_id(pool, &user_id).await?;

    if check_token.is_some() {
        let auth_token = AuthToken::from(check_token.unwrap());

        if ! auth_token.is_expired() {
            return Ok(auth_token)
        }
    }

    let auth_token = AuthToken::new(&user_id);

    repository::save_token(pool, &auth_token).await?;

    Ok(auth_token)
}
