use crate::products::model::ProductModel;
use crate::traits::HasQuantity;
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicProduct {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl From<ProductModel> for PublicProduct {
    fn from(product: ProductModel) -> Self {
        Self {
            id: product.id,
            name: product.name,
            slug: product.slug,
            price: product.price,
            quantity: product.quantity,
            configurable: product.configurable,
            is_active: product.is_active,
        }
    }
}

pub struct UpdateStockDto {
    pub product_id: i64,
    pub quantity: i32,
}

impl UpdateStockDto {
    pub fn new(product_id: i64, quantity: i32) -> Self {
        Self {
            product_id,
            quantity,
        }
    }
}

impl HasQuantity for UpdateStockDto {
    fn get_quantity(&self) -> i32 {
        self.quantity
    }
}
