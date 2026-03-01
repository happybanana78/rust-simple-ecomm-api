use crate::app::products::videos::model::ProductVideoModel;
use crate::errors::error::AppError;
use crate::utils::traits::IsRepository;
use sqlx::{Executor, PgPool};

pub struct ProductVideoRepository {
    pool: PgPool,
}

impl IsRepository for ProductVideoRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl ProductVideoRepository {
    pub async fn get_all_by_product(
        &self,
        product_id: i64,
    ) -> Result<Vec<ProductVideoModel>, AppError> {
        sqlx::query_as! {
            ProductVideoModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort
        FROM product_videos
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
    ) -> Result<Vec<ProductVideoModel>, AppError> {
        sqlx::query_as! {
            ProductVideoModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort
        FROM product_videos
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
