use std::collections::HashSet;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use crate::auth::dto::AuthToken;

#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthTokenModel {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
    pub scopes: Json<HashSet<String>>,
}

impl From<AuthTokenModel> for AuthToken {
    fn from(token: AuthTokenModel) -> Self {
        Self {
            token: token.token,
            user_id: token.user_id,
            expires_at: token.expires_at,
            scopes: token.scopes.0,
        }
    }
}
