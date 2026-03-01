use crate::admin::reviews::dto::ReviewApprovalStatus;
use crate::admin::reviews::filters::AdminReviewFilters;
use crate::admin::reviews::model::AdminReviewModel;
use crate::errors::error::AppError;
use crate::utils::pagination::Paginate;
use crate::utils::traits::IsRepository;
use sqlx::{Executor, PgPool, Postgres, QueryBuilder};

pub struct AdminReviewRepository {
    pool: PgPool,
}

impl IsRepository for AdminReviewRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl AdminReviewRepository {
    pub async fn index_paginated(
        &self,
        pagination: &Paginate,
        search: &Option<String>,
        filters: &AdminReviewFilters,
    ) -> Result<Vec<AdminReviewModel>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                product_reviews.id,
                product_reviews.user_id,
                product_reviews.product_id,
                product_reviews.title,
                product_reviews.content,
                product_reviews.rating,
                product_reviews.approval_status,
                product_reviews.created_at
            FROM product_reviews
        "#,
        );

        let mut has_where = false;

        // handle search
        if let Some(search) = search {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" (product_reviews.title ILIKE ");
            qb.push_bind(format!("%{}%", search));

            qb.push(" OR product_reviews.content ILIKE ");
            qb.push_bind(format!("%{}%", search));

            qb.push(")");
        }

        // user id
        if let Some(user_id) = filters.user_id {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" product_reviews.user_id = ");
            qb.push_bind(user_id);
        }

        // product id
        if let Some(product_id) = filters.product_id {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" product_reviews.product_id = ");
            qb.push_bind(product_id);
        }

        // rating
        if let Some(rating) = filters.rating {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" product_reviews.rating = ");
            qb.push_bind(rating);
        }

        // approval status
        if let Some(status) = &filters.approval_status {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }

            qb.push(" product_reviews.approval_status = ");
            qb.push_bind(status);
        }

        // handle pagination
        qb.push(" LIMIT ");
        qb.push_bind(pagination.limit);
        qb.push(" OFFSET ");
        qb.push_bind(pagination.get_offset());

        let query = qb.build_query_as::<AdminReviewModel>();

        query
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn show(&self, id: i64) -> Result<Option<AdminReviewModel>, AppError> {
        sqlx::query_as! {
            AdminReviewModel,
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
            WHERE id = $1;
            "#,
            id,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update_status(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        status: &ReviewApprovalStatus,
        id: i64,
    ) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            AdminProductModel,
            r#"
            UPDATE product_reviews
            SET approval_status = $1
            WHERE id = $2;
            "#,
            status as &ReviewApprovalStatus, id
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
            "DELETE FROM product_reviews WHERE id = $1;",
            id
        }
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
