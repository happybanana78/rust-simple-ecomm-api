use crate::cart::user_cart::model::{UserCartIdModel, UserCartModel};
use crate::errors::error::AppError;
use sqlx::PgPool;

pub struct UserCartRepository {
    pool: PgPool,
}

impl UserCartRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_cart_by_user_id(
        &self,
        user_id: &i64,
    ) -> Result<Option<UserCartModel>, AppError> {
        sqlx::query_as!(
            UserCartModel,
            r#"
        SELECT
            id,
            user_id AS "user_id!",
            total,
            created_at
        FROM cart
        WHERE user_id = $1;
        "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_cart_id(&self, user_id: &i64) -> Result<Option<UserCartIdModel>, AppError> {
        sqlx::query_as!(
            UserCartIdModel,
            r#"
        SELECT
            id
        FROM cart
        WHERE user_id = $1;
        "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn create_user_cart(&self, user_id: &i64) -> Result<UserCartModel, AppError> {
        sqlx::query_as!(
            UserCartModel,
            r#"
        INSERT INTO cart (user_id, total, created_at)
        VALUES ($1, 0, NOW())
        RETURNING id, user_id AS "user_id!", total, created_at;
        "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn delete_by_user_id(&self, user_id: &i64) -> Result<u64, AppError> {
        let result = sqlx::query!("DELETE FROM cart WHERE user_id = $1;", user_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
