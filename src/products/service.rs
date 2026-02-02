use super::model::Product;
use super::repository;
use sqlx::PgPool;
use crate::errors::error::AppError;
use crate::products::dto::{CreateProductCommand, UpdateProductCommand};

pub async fn index(pool: &PgPool) -> Result<Vec<Product>, AppError>
{
    repository::index(pool).await
}

pub async fn show(pool: &PgPool, id: i64) -> Result<Option<Product>, AppError>
{
    let product = repository::show(pool, id).await?;
    
    if product.is_none() {
        return Err(AppError::NotFound("Product not found".to_string()))
    }
    
    Ok(product)
}

pub async fn create(pool: &PgPool, cmd: CreateProductCommand) -> Result<Product, AppError>
{
    repository::create(pool, cmd).await
}

pub async fn update(pool: &PgPool, cmd: UpdateProductCommand, id: i64) -> Result<u64, AppError> {
    repository::update(pool, cmd, id).await
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, AppError> {
    repository::delete(pool, id).await
}
