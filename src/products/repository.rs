use super::model::ProductModel;
use crate::errors::error::AppError;
use sqlx::PgPool;

pub async fn index(pool: &PgPool) -> Result<Vec<ProductModel>, AppError> {
    sqlx::query_as! {
        ProductModel,
        "SELECT id, name, price, created_at FROM products;",
    }
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn show(pool: &PgPool, id: i64) -> Result<Option<ProductModel>, AppError> {
    sqlx::query_as! {
        ProductModel,
        "SELECT id, name, price, created_at FROM products WHERE id = $1;",
        id,
    }
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}
