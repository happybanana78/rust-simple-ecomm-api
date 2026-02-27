use super::model::AdminSafeUserModel;
use crate::admin::users::repository::AdminUserRepository;
use crate::errors::error::AppError;
use crate::traits::IsRepository;
use sqlx::PgPool;

pub struct AdminUserService {
    repository: AdminUserRepository,
}

impl AdminUserService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminUserRepository::new(pool),
        }
    }

    pub async fn get_one_safe(&self, id: i64) -> Result<AdminSafeUserModel, AppError> {
        let user = self.repository.show_safe(id).await?;

        match user {
            Some(user) => Ok(user),
            None => Err(AppError::NotFound("User not found".to_string())),
        }
    }
}
