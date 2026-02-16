use crate::errors::error::AppError;
use sqlx::PgPool;

pub trait IsRepository {
    type Repository;

    fn new(pool: PgPool) -> Self::Repository;

    fn get_pool(&self) -> &PgPool;

    async fn start_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, AppError> {
        self.get_pool().begin().await.map_err(AppError::Database)
    }

    async fn commit_transaction(
        &self,
        tx: sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), AppError> {
        tx.commit().await.map_err(AppError::Database)
    }
}

pub trait HasQuantity {
    fn get_quantity(&self) -> i32;

    fn is_safe_quantity(&self) -> bool {
        if self.get_quantity() < 0 {
            return false;
        }
        true
    }
}
