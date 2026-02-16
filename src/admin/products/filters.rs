use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFilters {
    pub price_min: Option<f64>,

    pub price_max: Option<f64>,

    pub in_stock: Option<bool>,

    pub is_active: Option<bool>,
}
