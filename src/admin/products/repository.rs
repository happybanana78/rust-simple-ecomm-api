use crate::admin::products::dto::{CreateProductCommand, UpdateProductCommand};
use crate::admin::products::model::AdminProductModel;
use crate::errors::error::AppError;
use sqlx::PgPool;

pub async fn index(pool: &PgPool) -> Result<Vec<AdminProductModel>, AppError> {
    sqlx::query_as! {
        AdminProductModel,
        "SELECT id, name, price, created_at FROM products;",
    }
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn show(pool: &PgPool, id: i64) -> Result<Option<AdminProductModel>, AppError> {
    sqlx::query_as! {
        AdminProductModel,
        "SELECT id, name, price, created_at FROM products WHERE id = $1;",
        id,
    }
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn create(
    pool: &PgPool,
    cmd: CreateProductCommand,
) -> Result<AdminProductModel, AppError> {
    sqlx::query_as! {
        AdminProductModel,
        "INSERT INTO products (name, price) VALUES ($1, $2) RETURNING *;",
        cmd.name, cmd.price,
    }
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn update(pool: &PgPool, cmd: UpdateProductCommand, id: i64) -> Result<u64, AppError> {
    let result = sqlx::query_as! {
        Product,
        "UPDATE products SET (name, price) = ($1, $2) WHERE id = $3;",
        cmd.name, cmd.price, id
    }
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, AppError> {
    let result = sqlx::query_as! {
        Product,
        "DELETE FROM products WHERE id = $1;",
        id
    }
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}
