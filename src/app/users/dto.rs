use crate::app::roles::dto::RoleEnum;
use crate::app::users::model::UserHashModel;
use chrono::{DateTime, Utc};

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

pub struct GuestToken(pub String);
