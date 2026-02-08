use crate::errors::error::AppError;
use crate::roles::dto::RoleEnum;
use crate::roles::repository;
use sqlx::{Acquire, PgPool, Postgres};
use std::str::FromStr;

pub async fn assign_role<'a, A>(
    connection: A,
    user_id: &i64,
    role: &RoleEnum,
) -> Result<(), AppError>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut conn = connection.acquire().await.map_err(AppError::Database)?;

    let role = repository::get_role_by_name(&mut *conn, role).await?;
    repository::assign_role(&mut *conn, user_id, &role.id).await?;
    Ok(())
}

pub async fn get_user_role(pool: &PgPool, user_id: &i64) -> Result<RoleEnum, AppError> {
    let role = repository::get_user_role(pool, user_id).await?;
    RoleEnum::from_str(&role.name)
}
