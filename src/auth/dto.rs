use crate::auth::model::{AuthTokenModel, UserModel};
use crate::auth::traits::Scope;
use crate::cart::user_cart::service as user_cart_service;
use crate::errors::error::AppError;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;
use validator::{Validate, ValidationError};

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

#[derive(Debug)]
pub struct AuthToken {
    pub token: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
    pub scopes: HashSet<String>,
}

impl AuthToken {
    pub fn new(user_id: &i64, scopes: HashSet<String>) -> Self {
        let token = Uuid::new_v4().to_string();

        AuthToken {
            token,
            user_id: user_id.clone(),
            expires_at: Utc::now() + Duration::days(2),
            scopes,
        }
    }

    pub fn has_scope<S: Scope>(&self, scope: S) -> bool {
        self.scopes.contains(scope.as_str())
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
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

#[derive(Clone, Copy)]
pub struct AuthUserId(pub i64);

impl AuthUserId {
    pub async fn get_cart_id(&self, pool: &PgPool) -> Result<i64, AppError> {
        user_cart_service::get_cart_id_by_user(pool, &self.0).await
    }
}

#[derive(Clone)]
pub struct AuthScopes(pub HashSet<String>);
