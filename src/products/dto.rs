use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,
}
