use sqlx::PgPool;
use crate::cart::model::CartModel;
use crate::errors::error::AppError;

pub async fn get_cart_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<CartModel>, AppError> {
    sqlx::query_as!(
        CartModel,
        "SELECT id, user_id, total, created_at FROM cart WHERE user_id = $1;",
        user_id
    )
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn create_cart(pool: &PgPool, user_id: &i64) -> Result<CartModel, AppError> {
    sqlx::query_as!(
        CartModel,
        "INSERT INTO cart (user_id, total, created_at) VALUES ($1, 0, NOW()) RETURNING *;",
        user_id
    )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)
}

pub async fn delete_by_user_id(pool: &PgPool, user_id: i64) -> Result<u64, AppError> {
    let result = sqlx::query_as!(
        CartModel,
        "DELETE FROM cart WHERE user_id = $1;",
        user_id
    )
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}
