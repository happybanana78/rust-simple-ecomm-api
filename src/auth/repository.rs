use super::model::{AuthTokenModel, UserModel};
use crate::auth::dto::{AuthToken, NewUser};
use crate::errors::error::AppError;
use sqlx::types::Json;
use sqlx::{Executor, Postgres};
use std::collections::HashSet;

#[derive(Clone)]
pub struct AuthRepository;

impl AuthRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn register<'e, E>(&self, executor: E, user: NewUser) -> Result<UserModel, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as::<_, UserModel>(
            "INSERT INTO users (username, email, password)
             VALUES ($1, $2, $3)
             RETURNING *;",
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .fetch_one(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn find_by_email<'e, E>(
        &self,
        executor: E,
        email: &str,
    ) -> Result<Option<UserModel>, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as! {
            UserModel,
            "SELECT id, username, email, password FROM users WHERE email = $1;",
            email
        }
        .fetch_optional(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn save_token<'e, E>(
        &self,
        executor: E,
        dto: &AuthToken,
    ) -> Result<AuthTokenModel, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let json_scopes = serde_json::to_value(&dto.scopes)
            .map_err(|_| AppError::Internal("failed to serialize scopes".into()))?;

        sqlx::query_as! {
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
        .fetch_one(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_token<'e, E>(
        &self,
        executor: E,
        token: String,
    ) -> Result<Option<AuthTokenModel>, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as! {
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
        .fetch_optional(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_token_by_user_id<'e, E>(
        &self,
        executor: E,
        user_id: &i64,
    ) -> Result<Option<AuthTokenModel>, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as! {
            AuthTokenModel,
            r#"
        SELECT
            id,
            token,
            user_id,
            expires_at,
            scopes AS "scopes: Json<HashSet<String>>"
        FROM personal_access_tokens
        WHERE user_id = $1
        ORDER BY expires_at DESC;
        "#,
            user_id
        }
        .fetch_optional(executor)
        .await
        .map_err(AppError::Database)
    }
}
