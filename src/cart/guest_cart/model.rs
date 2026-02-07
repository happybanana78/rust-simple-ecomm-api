use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct GuestCartModel {
    pub id: i64,
    pub user_hash_id: i64,
    pub total: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct GuestCartIdModel {
    pub id: i64,
}
