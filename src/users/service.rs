use crate::errors::error::AppError;
use crate::users::model::{UserHashModel, UserModel};
use crate::users::repository;
use sqlx::PgPool;

pub async fn get_user_by_id(pool: &PgPool, user_id: i64) -> Result<Option<UserModel>, AppError> {
    repository::get_user_by_id(pool, user_id).await
}

pub async fn get_user_hash(pool: &PgPool, hash: &str) -> Result<Option<UserHashModel>, AppError> {
    repository::get_user_hash(pool, hash).await
}
