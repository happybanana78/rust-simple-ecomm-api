use crate::admin::products::dto::{CreateProductCommand, UpdateProductCommand};
use crate::admin::products::filters::ProductFilters;
use crate::admin::products::model::AdminProductModel;
use crate::errors::error::AppError;
use crate::utils::pagination::Paginate;
use crate::utils::traits::IsRepository;
use sqlx::{Executor, PgPool, Postgres, QueryBuilder};

pub struct AdminProductRepository {
    pool: PgPool,
}

impl IsRepository for AdminProductRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl AdminProductRepository {
    pub async fn index(&self) -> Result<Vec<AdminProductModel>, AppError> {
        sqlx::query_as! {
            AdminProductModel,
            r#"
        SELECT
            id,
            name,
            slug,
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
                products.id,
                products.name,
                products.slug,
                products.price,
                products.quantity,
                products.configurable,
                products.is_active,
                products.created_at
            FROM products
        "#,
        );

        let mut has_where = false;

        // category
        if let Some(category) = filters.category {
            qb.push(
                " JOIN product_has_categories ON product_has_categories.product_id = products.id ",
            );

            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" product_has_categories.category_id = ");
            qb.push_bind(category);
        }

        // handle search
        if let Some(search) = search {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" products.name ILIKE ");
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
                qb.push(" products.quantity > 0 ");
            } else {
                qb.push(" products.quantity = 0 ");
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
                qb.push(" products.is_active IS TRUE ");
            } else {
                qb.push(" products.is_active IS FALSE ");
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

            qb.push(" products.price >= ");
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

            qb.push(" products.price <= ");
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
            slug,
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

    pub async fn create(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        cmd: &CreateProductCommand,
    ) -> Result<AdminProductModel, AppError> {
        sqlx::query_as! {
            AdminProductModel,
            r#"
        INSERT INTO products (name, slug, price, quantity, configurable, is_active)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *;
        "#,
            cmd.name, cmd.slug, cmd.price, cmd.quantity, cmd.configurable, cmd.is_active
        }
        .fetch_one(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        cmd: UpdateProductCommand,
        id: i64,
    ) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            AdminProductModel,
            r#"
        UPDATE products
        SET (name, slug, price, quantity, configurable, is_active) = ($1, $2, $3, $4, $5, $6)
        WHERE id = $7;
        "#,
            cmd.name, cmd.slug, cmd.price, cmd.quantity, cmd.configurable, cmd.is_active, id
        }
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn delete(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        id: i64,
    ) -> Result<u64, AppError> {
        let result = sqlx::query! {
            "DELETE FROM products WHERE id = $1;",
            id
        }
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn check_existence_by_slug(&self, slug: &str) -> Result<bool, AppError> {
        sqlx::query_scalar! {
            r#"
            SELECT EXISTS (
                SELECT 1 FROM products WHERE slug = $1
            ) AS "exists!";
            "#,
            slug,
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn detach_product_from_all_categories(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        product_id: i64,
    ) -> Result<u64, AppError> {
        let result = sqlx::query! {
            r#"
            DELETE FROM product_has_categories WHERE product_id = $1;
            "#,
            product_id
        }
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn attach_product_to_category(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        product_id: i64,
        category_id: i64,
    ) -> Result<u64, AppError> {
        let result = sqlx::query! {
            r#"
        INSERT INTO product_has_categories (category_id, product_id)
        VALUES ($1, $2);
        "#,
            category_id, product_id
        }
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
