use crate::admin::products::dto::{CreateProductCommand, UpdateProductCommand};
use crate::admin::products::model::AdminProductModel;
use crate::errors::error::AppError;
use crate::pagination::Paginate;
use sqlx::PgPool;

pub struct AdminProductRepository {
    pool: PgPool,
}

impl AdminProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn index(&self) -> Result<Vec<AdminProductModel>, AppError> {
        sqlx::query_as! {
            AdminProductModel,
            r#"
        SELECT
            id,
            name,
            price,
            quantity,
            configurable,
            is_active,
            created_at
        FROM products;
        "#,
        }
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn index_paginated(
        &self,
        pagination: &Paginate,
    ) -> Result<Vec<AdminProductModel>, AppError> {
        sqlx::query_as! {
            AdminProductModel,
            r#"
        SELECT
            id,
            name,
            price,
            quantity,
            configurable,
            is_active,
            created_at
        FROM products
        LIMIT $1
        OFFSET $2;
        "#,
            pagination.limit,
            pagination.offset
        }
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn show(&self, id: i64) -> Result<Option<AdminProductModel>, AppError> {
        sqlx::query_as! {
            AdminProductModel,
            r#"
        SELECT
            id,
            name,
            price,
            quantity,
            configurable,
            is_active,
            created_at
        FROM products
        WHERE id = $1;
        "#,
            id,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn create(&self, cmd: CreateProductCommand) -> Result<AdminProductModel, AppError> {
        sqlx::query_as! {
            AdminProductModel,
            r#"
        INSERT INTO products (name, price, quantity, configurable, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;
        "#,
            cmd.name, cmd.price, cmd.quantity, cmd.configurable, cmd.is_active
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(&self, cmd: UpdateProductCommand, id: i64) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            Product,
            r#"
        UPDATE products
        SET (name, price, quantity, configurable, is_active) = ($1, $2, $3, $4, $5)
        WHERE id = $6;
        "#,
            cmd.name, cmd.price, cmd.quantity, cmd.configurable, cmd.is_active, id
        }
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            Product,
            "DELETE FROM products WHERE id = $1;",
            id
        }
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
