use crate::app::roles::dto::RoleEnum;
use crate::app::roles::model::RoleModel;
use crate::errors::error::AppError;
use sqlx::{Executor, PgPool, Postgres};

#[derive(Clone)]
pub struct RoleRepository;

impl RoleRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_user_role(&self, pool: &PgPool, user_id: &i64) -> Result<RoleModel, AppError> {
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

    pub async fn get_role_by_name<'e, E>(
        &self,
        executor: E,
        role: &RoleEnum,
    ) -> Result<RoleModel, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as!(
            RoleModel,
            "SELECT id, name FROM roles WHERE name = $1;",
            role.as_str()
        )
        .fetch_one(executor)
        .await
        .map_err(AppError::Database)
    }

    pub async fn assign_role<'e, E>(
        &self,
        executor: E,
        user_id: &i64,
        role_id: &i64,
    ) -> Result<u64, AppError>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let result = sqlx::query!(
            r#"
        INSERT INTO user_has_roles (user_id, role_id)
        VALUES ($1, $2);
        "#,
            user_id,
            role_id
        )
        .execute(executor)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }
}
