use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ProductFilters {
    #[validate(range(min = 0.0))]
    pub price_min: Option<f64>,

    #[validate(range(min = 0.0))]
    pub price_max: Option<f64>,

    pub in_stock: Option<bool>,
}
