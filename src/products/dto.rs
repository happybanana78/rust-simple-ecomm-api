use crate::products::model::ProductModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicProduct {
    pub name: String,
    pub price: f64,
}

impl From<ProductModel> for PublicProduct {
    fn from(product: ProductModel) -> Self {
        Self {
            name: product.name,
            price: product.price,
        }
    }
}
