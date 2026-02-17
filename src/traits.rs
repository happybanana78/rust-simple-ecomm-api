use crate::errors::error::AppError;
use actix_multipart::form::tempfile::TempFile;
use actix_web::mime::Mime;
use bytes::Bytes;
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

pub trait UseStorage {
    fn upload(
        &self,
        path: &str,
        ext: &str,
        bytes: Bytes,
    ) -> impl Future<Output = Result<String, AppError>> + Send;

    fn upload_from_temp(
        &self,
        path: &str,
        temp_file: TempFile,
    ) -> impl Future<Output = Result<String, AppError>> + Send;

    fn delete(&self, path: &str) -> impl Future<Output = Result<(), AppError>> + Send;

    fn mime_to_extension(&self, mime: &Mime) -> String {
        mime.subtype().to_string()
    }
}
