use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::time::interval;
use validator::{Validate, ValidationError};
use crate::auth::model::{AuthTokenModel, UserModel};
use crate::auth::repository;
use crate::errors::error::AppError;

#[derive(Deserialize, Validate)]
#[validate(schema(function = "passwords_match"))]
pub struct RegisterDTO {
    #[validate(required, length(min = 3))]
    pub username: Option<String>,

    #[validate(required, email)]
    pub email: Option<String>,

    #[validate(required, length(min = 6))]
    pub password: Option<String>,

    #[validate(required, length(min = 6))]
    pub password_confirmation: Option<String>,
}

fn passwords_match(dto: &RegisterDTO) -> Result<(), ValidationError> {
    match (&dto.password, &dto.password_confirmation) {
        (Some(pwd), Some(pwd_confirm)) if pwd == pwd_confirm => Ok(()),
        _ => {
            let mut err = ValidationError::new("password_mismatch");
            err.message = Some("passwords do not match".into());
            Err(err)
        }
    }
}

pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<RegisterDTO> for RegisterCommand {
    type Error = AppError;

    fn try_from(dto: RegisterDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            username: dto.username.unwrap(),
            email: dto.email.unwrap(),
            password: dto.password.unwrap(),
        })
    }
}

#[derive(Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(required, length(min = 1))]
    pub email: Option<String>,

    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

impl TryFrom<LoginDTO> for LoginCommand {
    type Error = AppError;

    fn try_from(dto: LoginDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            email: dto.email.unwrap(),
            password: dto.password.unwrap(),
        })
    }
}

pub struct AuthToken {
    pub token: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}

impl AuthToken {
    pub async fn new(user_id: &i64) -> Self {
        let mut interval = interval(Duration::from_secs(1));
        interval.tick().await;

        let token = Utc::now().to_string();

        AuthToken {
            token,
            user_id: user_id.clone(),
            expires_at: Utc::now() + chrono::Duration::seconds(60 * 60 * 24), // 1 day
        }
    }

    pub async fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub async fn get_token(pool: &PgPool, token: String) -> Result<Self, AppError> {
        let auth_token = repository::get_token(pool, token).await
            .map_err(|_| AppError::Unauthorized("invalid token".to_string()))?;

        Ok(
            AuthToken {
                token: auth_token.token,
                user_id: auth_token.user_id,
                expires_at: auth_token.expires_at, // 1 day
            }
        )
    }

    pub async fn save_token(pool: &PgPool, user_id: i64) -> Result<Self, AppError> {
        let check_token = repository::get_token_by_user_id(pool, &user_id).await?;

        if check_token.is_some() {
            let auth_token = Self::from(check_token.unwrap());

            if ! auth_token.is_expired().await {
                return Ok(auth_token)
            }
        }

        let auth_token = Self::new(&user_id).await;

        repository::save_token(pool, &auth_token).await?;

        Ok(auth_token)
    }
}

#[derive(Serialize)]
pub struct PublicAuthToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl From<AuthToken> for PublicAuthToken {
    fn from(token: AuthToken) -> Self {
        Self {
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}

impl From<AuthTokenModel> for PublicAuthToken {
    fn from(token: AuthTokenModel) -> Self {
        Self {
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}

#[derive(Serialize)]
pub struct PublicUser {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

impl From<UserModel> for PublicUser {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
