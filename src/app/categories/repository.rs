use crate::app::categories::model::CategoryModel;
use crate::errors::error::AppError;
use crate::pagination::Paginate;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn index_paginated(
        &self,
        pagination: &Paginate,
        search: &Option<String>,
    ) -> Result<Vec<CategoryModel>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(
            r#"
            SELECT
                id,
                name,
                slug,
                is_active
            FROM categories
            WHERE is_active = true
            "#,
        );

        // handle search
        if let Some(search) = search {
            qb.push(" AND categories.name ILIKE ");
            qb.push_bind(format!("%{}%", search));
        }

        // handle pagination
        qb.push(" LIMIT ");
        qb.push_bind(pagination.limit);
        qb.push(" OFFSET ");
        qb.push_bind(pagination.get_offset());

        let query = qb.build_query_as::<CategoryModel>();

        query
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn show(&self, slug: &str) -> Result<Option<CategoryModel>, AppError> {
        sqlx::query_as! {
            CategoryModel,
            r#"
        SELECT
            id,
            name,
            slug,
            is_active
        FROM categories
        WHERE slug = $1 AND is_active = true;
        "#,
            slug,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
