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

pub async fn get_product_stock(pool: &PgPool, product_id: &i64) -> Result<i32, AppError> {
    sqlx::query_scalar!(
        r#"
        SELECT
            quantity
        FROM products
        WHERE id = $1;
        "#,
        product_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn update_product_stock(
    pool: &PgPool,
    product_id: &i64,
    new_quantity: &i32,
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE products
        SET quantity = $1 WHERE id = $2;
        "#,
        new_quantity,
        product_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}
