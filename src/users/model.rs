use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(FromRow)]
pub struct UserHashModel {
    pub id: i64,
    pub hash: String,
    pub expires_at: DateTime<Utc>,
}
