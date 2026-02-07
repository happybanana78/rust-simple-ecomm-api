use crate::cart::guest_cart::service;
use crate::errors::error::AppError;
use crate::roles::dto::RoleEnum;
use crate::users::model::UserHashModel;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub struct UserScopes {
    pub scopes: Vec<String>,
}

impl UserScopes {
    pub fn by_role(role: RoleEnum) -> Self {
        match role {
            RoleEnum::Admin => Self {
                scopes: vec![
                    "create-user".to_string(),
                    "read-user".to_string(),
                    "update-user".to_string(),
                    "delete-user".to_string(),
                    "list-users".to_string(),
                ],
            },
            RoleEnum::User => Self {
                scopes: vec!["read-user".to_string()],
            },
        }
    }
}

pub struct GuestDto {
    pub id: i64,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
}

impl From<UserHashModel> for GuestDto {
    fn from(model: UserHashModel) -> Self {
        Self {
            id: model.id,
            hash: model.hash,
            expires_at: model.expires_at,
        }
    }
}

impl GuestDto {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[derive(Clone)]
pub struct GuestToken(pub String);

impl GuestToken {
    pub async fn get_cart_id(&self, pool: &PgPool) -> Result<i64, AppError> {
        service::get_cart_id_by_hash(pool, &self.0).await
    }
}
