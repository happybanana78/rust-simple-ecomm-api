use crate::app::users::model::{UserHashModel, UserModel};
use crate::app::users::repository::UserRepository;
use crate::errors::error::AppError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: UserRepository::new(pool),
        }
    }

    pub async fn get_user_by_id(&self, user_id: &i64) -> Result<Option<UserModel>, AppError> {
        self.repository.get_user_by_id(user_id).await
    }

    pub async fn get_user_hash(&self, hash: &str) -> Result<Option<UserHashModel>, AppError> {
        self.repository.get_user_hash(hash).await
    }
}
