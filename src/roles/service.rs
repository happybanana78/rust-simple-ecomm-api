use std::str::FromStr;
use sqlx::PgPool;
use crate::errors::error::AppError;
use crate::roles::dto::RoleEnum;
use crate::roles::repository;

pub async fn assign_role(pool: &PgPool, user_id: &i64, role: &RoleEnum) -> Result<(), AppError> {
    let role = repository::get_role_by_name(pool, role).await?;
    repository::assign_role(pool, user_id, &role.id).await?;
    Ok(())
}

pub async fn get_user_role(pool: &PgPool, user_id: &i64) -> Result<RoleEnum, AppError> {
    let role = repository::get_user_role(pool, user_id).await?;
    RoleEnum::from_str(&role.name)
}
