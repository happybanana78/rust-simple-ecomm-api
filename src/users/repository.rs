use crate::errors::error::AppError;
use crate::users::model::{UserHashModel, UserModel};
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_id(&self, user_id: &i64) -> Result<Option<UserModel>, AppError> {
        sqlx::query_as!(
            UserModel,
            "SELECT id, username, email FROM users WHERE id = $1;",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_user_hash(&self, hash: &str) -> Result<Option<UserHashModel>, AppError> {
        sqlx::query_as!(
            UserHashModel,
            "SELECT id, hash, expires_at FROM user_hashes WHERE hash = $1;",
            hash
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
