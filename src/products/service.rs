use super::model::ProductModel;
use super::repository;
use crate::errors::error::AppError;
use sqlx::PgPool;

pub async fn index(pool: &PgPool) -> Result<Vec<ProductModel>, AppError> {
    repository::index(pool).await
}

pub async fn show(pool: &PgPool, id: i64) -> Result<ProductModel, AppError> {
    let product = repository::show(pool, id).await?;

    if product.is_none() {
        return Err(AppError::NotFound("Product not found".to_string()));
    }

    Ok(product.unwrap())
}

pub async fn check(pool: &PgPool, id: &i64) -> Result<bool, AppError> {
    let product = repository::check_exist_and_active(pool, id).await?;

    match product {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
