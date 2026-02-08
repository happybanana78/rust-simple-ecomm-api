use crate::products::model::ProductModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicProduct {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl From<ProductModel> for PublicProduct {
    fn from(product: ProductModel) -> Self {
        Self {
            name: product.name,
            price: product.price,
            quantity: product.quantity,
            configurable: product.configurable,
            is_active: product.is_active,
        }
    }
}
