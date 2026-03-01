use crate::admin::users::model::AdminSafeUserModel;
use crate::errors::error::AppError;
use crate::utils::traits::IsRepository;
use sqlx::PgPool;

pub struct AdminUserRepository {
    pool: PgPool,
}

impl IsRepository for AdminUserRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl AdminUserRepository {
    pub async fn show_safe(&self, id: i64) -> Result<Option<AdminSafeUserModel>, AppError> {
        sqlx::query_as! {
            AdminSafeUserModel,
            r#"
            SELECT
                id,
                username,
                email
            FROM users
            WHERE id = $1;
            "#,
            id,
        }
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
