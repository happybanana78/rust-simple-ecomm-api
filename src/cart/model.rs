use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct CartModel {
    pub id: i64,
    pub user_id: Option<i64>,
    pub total: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
pub struct CartItemModel {
    pub id: i64,
    pub cart_id: i64,
    pub product_id: i64,
    pub price: f64,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}
