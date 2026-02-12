use chrono::{DateTime, Duration, Utc};
use fake::Dummy;
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::name::en::Name;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminProductModel {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Dummy)]
pub struct AdminProductDummy {
    #[dummy(faker = "Name()")]
    pub name: String,

    #[dummy(faker = "10.0..200.0")]
    pub price: f64,

    #[dummy(faker = "12..200")]
    pub quantity: i32,

    pub configurable: bool,

    pub is_active: bool,

    #[dummy(faker = "DateTimeBetween(Utc::now() - Duration::days(365), Utc::now())")]
    pub created_at: DateTime<Utc>,
}
