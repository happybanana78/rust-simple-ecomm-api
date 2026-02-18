use crate::traits::HasId;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ProductModel {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl HasId for ProductModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}

#[derive(FromRow)]
pub struct ProductIdModel {
    pub id: i64,
}
