use crate::admin::products::images::dto::CreateProductImageCommand;
use crate::admin::products::images::model::AdminProductImageModel;
use crate::errors::error::AppError;
use crate::products::images::model::ProductImageModel;
use crate::traits::IsRepository;
use sqlx::{Executor, PgPool, Postgres};

pub struct ProductImageRepository {
    pool: PgPool,
}

impl IsRepository for ProductImageRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl ProductImageRepository {
    pub async fn get_all_by_product(
        &self,
        product_id: i64,
    ) -> Result<Vec<ProductImageModel>, AppError> {
        sqlx::query_as! {
            ProductImageModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort
        FROM product_images
        WHERE product_id = $1;
        "#,
            product_id
        }
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_all_for_multiple_products(
        &self,
        product_ids: &Vec<i64>,
    ) -> Result<Vec<ProductImageModel>, AppError> {
        sqlx::query_as! {
            ProductImageModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort
        FROM product_images
        WHERE product_id = ANY($1)
        ORDER BY sort;
        "#,
            &product_ids
        }
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
