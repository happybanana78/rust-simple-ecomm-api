use crate::traits::HasId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductImageModel {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: i32,
}

impl HasId for ProductImageModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}
