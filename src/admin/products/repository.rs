use crate::admin::products::dto::{CreateProductCommand, UpdateProductCommand};
use crate::admin::products::filters::ProductFilters;
use crate::admin::products::model::AdminProductModel;
use crate::errors::error::AppError;
use crate::pagination::Paginate;
use sqlx::{PgPool, Postgres, QueryBuilder};

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
        search: &Option<String>,
        filters: &ProductFilters,
    ) -> Result<Vec<AdminProductModel>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
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
        "#,
        );

        let mut has_where = false;

        // handle search
        if let Some(search) = search {
            qb.push(" WHERE name ILIKE ");
            qb.push_bind(format!("%{}%", search));
        }

        // in stock
        if let Some(in_stock) = filters.in_stock {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            if in_stock {
                qb.push(" quantity > 0");
            } else {
                qb.push(" quantity = 0");
            }
        }

        // is active
        if let Some(is_active) = filters.is_active {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            if is_active {
                qb.push(" is_active IS TRUE ");
            } else {
                qb.push(" is_active IS FALSE ");
            }
        }

        // min price
        if let Some(min_price) = filters.price_min {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" price >= ");
            qb.push_bind(min_price);
        }

        // max price
        if let Some(max_price) = filters.price_max {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" price <= ");
            qb.push_bind(max_price);
        }

        // handle pagination
        qb.push(" LIMIT ");
        qb.push_bind(pagination.limit);
        qb.push(" OFFSET ");
        qb.push_bind(pagination.get_offset());

        let query = qb.build_query_as::<AdminProductModel>();

        query
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
            AdminProductModel,
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
        let result = sqlx::query! {
            "DELETE FROM products WHERE id = $1;",
            id
        }
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn check_existence_by_name(&self, name: &str) -> Result<bool, AppError> {
        sqlx::query_scalar! {
            r#"
            SELECT EXISTS (
                SELECT 1 FROM products WHERE name = $1
            ) AS "exists!";
            "#,
            name,
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
