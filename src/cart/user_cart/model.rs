use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct UserCartModel {
    pub id: i64,
    pub user_id: i64,
    pub total: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct UserCartIdModel {
    pub id: i64,
}
