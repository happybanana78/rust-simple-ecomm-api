use crate::admin::categories::dto::{CreateCategoryCommand, UpdateCategoryCommand};
use crate::admin::categories::filters::CategoryFilters;
use crate::admin::categories::model::AdminCategoryModel;
use crate::errors::error::AppError;
use crate::pagination::Paginate;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub struct AdminCategoryRepository {
    pool: PgPool,
}

impl AdminCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn index(&self) -> Result<Vec<AdminCategoryModel>, AppError> {
        sqlx::query_as! {
            AdminCategoryModel,
            r#"
        SELECT
            id,
            name,
            slug,
            is_active,
            created_at
        FROM categories;
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
        filters: &CategoryFilters,
    ) -> Result<Vec<AdminCategoryModel>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id,
                name,
                slug,
                is_active,
                created_at
            FROM categories
        "#,
        );

        let mut has_where = false;

        // handle search
        if let Some(search) = search {
            qb.push(" WHERE name ILIKE ");
            qb.push_bind(format!("%{}%", search));
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

        // handle pagination
        qb.push(" LIMIT ");
        qb.push_bind(pagination.limit);
        qb.push(" OFFSET ");
        qb.push_bind(pagination.get_offset());

        let query = qb.build_query_as::<AdminCategoryModel>();

        query
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn show(&self, id: i64) -> Result<Option<AdminCategoryModel>, AppError> {
        sqlx::query_as! {
            AdminCategoryModel,
            r#"
        SELECT
            id,
            name,
            slug,
            is_active,
            created_at
        FROM categories
        WHERE id = $1;
        "#,
            id,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn create(&self, cmd: CreateCategoryCommand) -> Result<AdminCategoryModel, AppError> {
        sqlx::query_as! {
            AdminCategoryModel,
            r#"
        INSERT INTO categories (name, slug, is_active)
        VALUES ($1, $2, $3)
        RETURNING *;
        "#,
            cmd.name, cmd.slug, cmd.is_active
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn update(&self, cmd: UpdateCategoryCommand, id: i64) -> Result<u64, AppError> {
        let result = sqlx::query_as! {
            AdminCategoryModel,
            r#"
        UPDATE categories
        SET (name, slug, is_active) = ($1, $2, $3)
        WHERE id = $4;
        "#,
            cmd.name, cmd.slug, cmd.is_active, id
        }
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        let result = sqlx::query! {
            "DELETE FROM categories WHERE id = $1;",
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
                SELECT 1 FROM categories WHERE name = $1
            ) AS "exists!";
            "#,
            name,
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn check_existence_by_slug(&self, slug: &str) -> Result<bool, AppError> {
        sqlx::query_scalar! {
            r#"
            SELECT EXISTS (
                SELECT 1 FROM categories WHERE slug = $1
            ) AS "exists!";
            "#,
            slug,
        }
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
