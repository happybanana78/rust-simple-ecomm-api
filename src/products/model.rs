use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProductModel {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct ProductIdModel {
    pub id: i64,
}
