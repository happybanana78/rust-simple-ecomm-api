use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow)]
pub struct AuthTokenModel {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}
