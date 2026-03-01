use crate::admin::products::images::dto::CreateProductImageCommand;
use crate::admin::products::images::model::{
    AdminProductImageModel, AdminProductImageOnlySortModel,
};
use crate::errors::error::AppError;
use crate::utils::traits::IsRepository;
use bigdecimal::BigDecimal;
use sqlx::{Executor, PgPool, Postgres};

pub struct AdminProductImageRepository {
    pool: PgPool,
}

impl IsRepository for AdminProductImageRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl AdminProductImageRepository {
    pub async fn get_all_by_product(
        &self,
        product_id: i64,
    ) -> Result<Vec<AdminProductImageModel>, AppError> {
        sqlx::query_as! {
            AdminProductImageModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort,
            deleted_at,
            created_at
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
        product_ids: Vec<i64>,
    ) -> Result<Vec<AdminProductImageModel>, AppError> {
        sqlx::query_as! {
            AdminProductImageModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort,
            deleted_at,
            created_at
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

    pub async fn show(&self, id: i64) -> Result<Option<AdminProductImageModel>, AppError> {
        sqlx::query_as! {
            AdminProductImageModel,
            r#"
        SELECT
            id,
            product_id,
            url,
            alt,
            is_main,
            sort,
            deleted_at,
            created_at
        FROM product_images
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
        cmd: &CreateProductImageCommand,
    ) -> Result<AdminProductImageModel, AppError> {
        sqlx::query_as! {
            AdminProductImageModel,
            r#"
        INSERT INTO product_images (product_id, url, alt, is_main, sort)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, product_id, url, alt, is_main, sort, deleted_at, created_at;
        "#,
            cmd.product_id, cmd.url, cmd.alt, cmd.is_main, cmd.sort
        }
        .fetch_one(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update_sort(&self, id: i64, sort: BigDecimal) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            AdminProductImageModel,
            r#"
            UPDATE product_images
            SET sort = $1
            WHERE id = $2;
            "#,
            sort,
            id,
        }
        .execute(&self.pool)
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
            "DELETE FROM product_images WHERE id = $1;",
            id
        }
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn get_total_count(&self, product_id: i64) -> Result<i64, AppError> {
        sqlx::query_scalar! {
            r#"
            SELECT COUNT(*) AS "count!"
            FROM product_images
            WHERE product_id = $1
            AND deleted_at IS NULL;
            "#,
            product_id,
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_images_only_sort(
        &self,
        product_id: i64,
    ) -> Result<Vec<AdminProductImageOnlySortModel>, AppError> {
        sqlx::query_as! {
            AdminProductImageOnlySortModel,
            r#"
            SELECT id, sort
            FROM product_images
            WHERE product_id = $1
            ORDER BY sort;
            "#,
            product_id,
        }
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_last_sort(&self, product_id: i64) -> Result<Option<BigDecimal>, AppError> {
        sqlx::query_scalar! {
            r#"
            SELECT sort
            FROM product_images
            WHERE product_id = $1
            AND deleted_at IS NULL
            ORDER BY sort DESC
            LIMIT 1;
            "#,
            product_id,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn reset_is_main(&self, product_id: i64) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            AdminProductImageModel,
            r#"
            UPDATE product_images
            SET is_main = false
            WHERE product_id = $1
            AND is_main = true;
            "#,
            product_id,
        }
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
