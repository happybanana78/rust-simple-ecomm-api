use std::collections::HashSet;
use sqlx::PgPool;
use sqlx::types::Json;
use crate::auth::dto::{AuthToken, NewUser};
use crate::errors::error::AppError;
use super::model::{AuthTokenModel, UserModel};

pub async fn register(pool: &PgPool, user: NewUser) -> Result<UserModel, AppError> {
    sqlx::query_as!{
        UserModel,
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *;",
        user.username, user.email, user.hashed_password
    }
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<UserModel>, AppError> {
    sqlx::query_as!{
        UserModel,
        "SELECT id, username, email, password FROM users WHERE email = $1;",
        email
    }
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn save_token(pool: &PgPool, dto: &AuthToken) -> Result<AuthTokenModel, AppError> {
    let json_scopes = serde_json::to_value(&dto.scopes)
        .map_err(|_| AppError::Internal("failed to serialize scopes".into()))?;

    sqlx::query_as!{
        AuthTokenModel,
        r#"
        INSERT INTO personal_access_tokens (token, user_id, expires_at, scopes)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            token,
            user_id,
            expires_at,
            scopes AS "scopes: Json<HashSet<String>>"
        "#,
        dto.token,
        dto.user_id,
        dto.expires_at,
        json_scopes,
    }
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn get_token(pool: &PgPool, token: String) -> Result<AuthTokenModel, AppError> {
    sqlx::query_as!{
        AuthTokenModel,
        r#"
        SELECT
            id,
            token,
            user_id,
            expires_at,
            scopes AS "scopes: Json<HashSet<String>>"
        FROM personal_access_tokens
        WHERE token = $1;
        "#,
        token
    }
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn get_token_by_user_id(pool: &PgPool, user_id: &i64) -> Result<Option<AuthTokenModel>, AppError> {
    sqlx::query_as!{
        AuthTokenModel,
        r#"
        SELECT
            id,
            token,
            user_id,
            expires_at,
            scopes AS "scopes: Json<HashSet<String>>"
        FROM personal_access_tokens
        WHERE user_id = $1;
        "#,
        user_id
    }
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)
}
