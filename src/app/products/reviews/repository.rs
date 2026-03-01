use crate::admin::reviews::dto::ReviewApprovalStatus;
use crate::app::products::reviews::dto::CreateProductReviewCommand;
use crate::app::products::reviews::model::ProductReviewModel;
use crate::errors::error::AppError;
use crate::traits::IsRepository;
use sqlx::{Executor, PgPool, Postgres};

pub struct ProductReviewRepository {
    pool: PgPool,
}

impl IsRepository for ProductReviewRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl ProductReviewRepository {
    pub async fn get_all_by_product(
        &self,
        product_id: i64,
    ) -> Result<Vec<ProductReviewModel>, AppError> {
        sqlx::query_as! {
            ProductReviewModel,
            r#"
            SELECT
                id,
                user_id,
                product_id,
                title,
                content,
                rating,
                approval_status as "approval_status: ReviewApprovalStatus",
                created_at
            FROM product_reviews
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
    ) -> Result<Vec<ProductReviewModel>, AppError> {
        sqlx::query_as! {
            ProductReviewModel,
            r#"
            SELECT
                id,
                user_id,
                product_id,
                title,
                content,
                rating,
                approval_status as "approval_status: ReviewApprovalStatus",
                created_at
            FROM product_reviews
            WHERE product_id = ANY($1);
            "#,
            &product_ids
        }
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn create(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        cmd: CreateProductReviewCommand,
    ) -> Result<ProductReviewModel, AppError> {
        sqlx::query_as! {
            ProductReviewModel,
            r#"
            INSERT INTO product_reviews (user_id, product_id, title, content, rating, approval_status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, product_id, title, content, rating, approval_status as "approval_status: ReviewApprovalStatus", created_at;
            "#,
            cmd.user_id,
            cmd.product_id,
            cmd.title,
            cmd.content,
            cmd.rating,
            cmd.status as ReviewApprovalStatus,
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
