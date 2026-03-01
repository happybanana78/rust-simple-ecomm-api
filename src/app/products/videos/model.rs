use crate::utils::traits::HasId;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductVideoModel {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: BigDecimal,
}

impl HasId for ProductVideoModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}
