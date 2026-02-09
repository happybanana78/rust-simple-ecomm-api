use crate::errors::error::AppError;
use crate::roles::dto::RoleEnum;
use crate::roles::repository::RoleRepository;
use sqlx::{Acquire, PgPool, Postgres};
use std::str::FromStr;

#[derive(Clone)]
pub struct RoleService {
    pub pool: PgPool,
    pub repository: RoleRepository,
}

impl RoleService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            repository: RoleRepository::new(),
        }
    }

    pub async fn assign_role<'a, A>(
        &self,
        connection: A,
        user_id: &i64,
        role: &RoleEnum,
    ) -> Result<(), AppError>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut conn = connection.acquire().await.map_err(AppError::Database)?;

        let role = &self.repository.get_role_by_name(&mut *conn, role).await?;
        &self
            .repository
            .assign_role(&mut *conn, user_id, &role.id)
            .await?;
        Ok(())
    }

    pub async fn get_user_role(&self, user_id: &i64) -> Result<RoleEnum, AppError> {
        let role = &self.repository.get_user_role(&self.pool, user_id).await?;
        RoleEnum::from_str(&role.name)
    }
}
