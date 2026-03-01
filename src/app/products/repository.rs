use super::model::{ProductIdModel, ProductModel};
use crate::app::products::filters::ProductFilters;
use crate::errors::error::AppError;
use crate::utils::pagination::Paginate;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn index_paginated(
        &self,
        pagination: &Paginate,
        search: &Option<String>,
        filters: &ProductFilters,
    ) -> Result<Vec<ProductModel>, AppError> {
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
            WHERE is_active = true
        "#,
        );

        // category
        if let Some(category) = filters.category {
            qb.push(
                " JOIN product_has_categories ON product_has_categories.product_id = products.id ",
            );

            qb.push(" AND product_has_categories.category_id = ");
            qb.push_bind(category);
        }

        // handle search
        if let Some(search) = search {
            qb.push(" AND products.name ILIKE ");
            qb.push_bind(format!("%{}%", search));
        }

        // min price
        if let Some(min_price) = filters.price_min {
            qb.push(" AND products.price >= ");
            qb.push_bind(min_price);
        }

        // max price
        if let Some(max_price) = filters.price_max {
            qb.push(" AND products.price <= ");
            qb.push_bind(max_price);
        }

        // handle pagination
        qb.push(" LIMIT ");
        qb.push_bind(pagination.limit);
        qb.push(" OFFSET ");
        qb.push_bind(pagination.get_offset());

        let query = qb.build_query_as::<ProductModel>();

        query
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn show(&self, slug: &str) -> Result<Option<ProductModel>, AppError> {
        sqlx::query_as! {
            ProductModel,
            r#"
        SELECT
            id,
            name,
            slug,
            price,
            quantity,
            configurable,
            is_active
        FROM products
        WHERE slug = $1 AND is_active = true;
        "#,
            slug,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn check_exist_and_active(
        &self,
        id: i64,
    ) -> Result<Option<ProductIdModel>, AppError> {
        sqlx::query_as! {
            ProductIdModel,
            "SELECT id FROM products WHERE id = $1 AND is_active = true;",
            id,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_product_stock(&self, product_id: &i64) -> Result<i32, AppError> {
        sqlx::query_scalar!(
            r#"
        SELECT
            quantity
        FROM products
        WHERE id = $1;
        "#,
            product_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update_product_stock(
        &self,
        product_id: i64,
        new_quantity: i32,
    ) -> Result<u64, AppError> {
        let result = sqlx::query!(
            r#"
        UPDATE products
        SET quantity = $1 WHERE id = $2;
        "#,
            new_quantity,
            product_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
