use sqlx::PgPool;
use crate::errors::error::AppError;
use crate::users::model::UserModel;
use crate::users::repository;

pub async fn get_user_by_id(pool: &PgPool, user_id: i64) -> Result<Option<UserModel>, AppError> {
    repository::get_user_by_id(pool, user_id).await
}
