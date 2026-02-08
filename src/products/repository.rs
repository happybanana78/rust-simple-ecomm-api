use super::model::{ProductIdModel, ProductModel};
use crate::errors::error::AppError;
use sqlx::PgPool;

pub async fn index(pool: &PgPool) -> Result<Vec<ProductModel>, AppError> {
    sqlx::query_as! {
        ProductModel,
        r#"
        SELECT
            id,
            name,
            price,
            quantity,
            configurable,
            is_active
        FROM products
        WHERE is_active = true;
        "#,
    }
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn show(pool: &PgPool, id: i64) -> Result<Option<ProductModel>, AppError> {
    sqlx::query_as! {
        ProductModel,
        r#"
        SELECT
            id,
            name,
            price,
            quantity,
            configurable,
            is_active
        FROM products
        WHERE id = $1 AND is_active = true;
        "#,
        id,
    }
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn check_exist_and_active(
    pool: &PgPool,
    id: &i64,
) -> Result<Option<ProductIdModel>, AppError> {
    sqlx::query_as! {
        ProductIdModel,
        "SELECT id FROM products WHERE id = $1 AND is_active = true;",
        id,
    }
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}
