use super::model::Product;
use super::repository;
use super::dto;
use sqlx::PgPool;

pub async fn index(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error>
{
    repository::index(pool).await
}

pub async fn show(pool: &PgPool, id: i32) -> Result<Option<Product>, sqlx::Error>
{
    repository::show(pool, id).await
}

pub async fn create(pool: &PgPool, dto: dto::CreateProductDTO) -> Result<Product, sqlx::Error>
{
    repository::create(pool, dto).await
}

pub async fn update(pool: &PgPool, dto: dto::UpdateProductDTO, id: i32) -> Result<u64, sqlx::Error> {
    repository::update(pool, dto, id).await
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    repository::delete(pool, id).await
}
