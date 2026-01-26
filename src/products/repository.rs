use sqlx::PgPool;
use crate::products::dto;
use super::model::Product;

pub async fn index(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    sqlx::query_as!{
        Product,
        "SELECT id, name, price, created_at FROM products;",
    }
        .fetch_all(pool)
        .await
}

pub async fn show(pool: &PgPool, id: i32) -> Result<Option<Product>, sqlx::Error> {
    sqlx::query_as!{
        Product,
        "SELECT id, name, price, created_at FROM products WHERE id = $1;",
        id,
    }
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &PgPool, dto: dto::CreateProductDTO) -> Result<Product, sqlx::Error> {
    sqlx::query_as!{
        Product,
        "INSERT INTO products (name, price) VALUES ($1, $2) RETURNING *;",
        dto.name, dto.price,
    }
        .fetch_one(pool)
        .await
}

pub async fn update(pool: &PgPool, dto: dto::UpdateProductDTO, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_as!{
        Product,
        "UPDATE products SET (name, price) = ($1, $2) WHERE id = $3;",
        dto.name, dto.price, id
    }
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_as!{
        Product,
        "DELETE FROM products WHERE id = $1;",
        id
    }
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
