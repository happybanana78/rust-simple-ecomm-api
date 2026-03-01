use crate::app::cart::guest_cart::model::{GuestCartIdModel, GuestCartModel};
use crate::errors::error::AppError;
use sqlx::PgPool;

pub struct GuestCartRepository {
    pool: PgPool,
}

impl GuestCartRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_cart_by_user_hash(
        &self,
        hash_id: &i64,
    ) -> Result<Option<GuestCartModel>, AppError> {
        sqlx::query_as!(
            GuestCartModel,
            r#"
        SELECT
            id,
            user_hash_id AS "user_hash_id!",
            total,
            created_at
        FROM cart
        WHERE user_hash_id = $1;
        "#,
            hash_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_cart_id(&self, hash_id: &i64) -> Result<Option<GuestCartIdModel>, AppError> {
        sqlx::query_as!(
            GuestCartIdModel,
            r#"
        SELECT
            id
        FROM cart
        WHERE user_hash_id = $1;
        "#,
            hash_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn create_hash_cart(&self, hash_id: &i64) -> Result<GuestCartModel, AppError> {
        sqlx::query_as!(
            GuestCartModel,
            r#"
        INSERT INTO cart (user_hash_id, total, created_at)
        VALUES ($1, 0, NOW())
        RETURNING id, user_hash_id AS "user_hash_id!", total, created_at;
        "#,
            hash_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn delete_by_hash_id(&self, hash_id: &i64) -> Result<u64, AppError> {
        let result = sqlx::query!("DELETE FROM cart WHERE user_hash_id = $1;", hash_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
