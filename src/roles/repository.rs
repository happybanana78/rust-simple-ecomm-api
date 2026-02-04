use crate::errors::error::AppError;
use crate::roles::dto::RoleEnum;
use crate::roles::model::RoleModel;
use sqlx::PgPool;

pub async fn get_user_role(pool: &PgPool, user_id: &i64) -> Result<RoleModel, AppError> {
    sqlx::query_as!(
        RoleModel,
        r#"
        SELECT 
            roles.id, 
            name 
        FROM roles 
        INNER JOIN user_has_roles ON roles.id = user_has_roles.role_id
        WHERE user_id = $1;
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn get_role_by_name(pool: &PgPool, role: &RoleEnum) -> Result<RoleModel, AppError> {
    sqlx::query_as!(
        RoleModel,
        "SELECT id, name FROM roles WHERE name = $1;",
        role.as_str()
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn assign_role(pool: &PgPool, user_id: &i64, role_id: &i64) -> Result<u64, AppError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO user_has_roles (user_id, role_id)
        VALUES ($1, $2);
        "#,
        user_id,
        role_id
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(result.rows_affected())
}
