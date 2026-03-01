use crate::app::products::images::model::ProductImageModel;
use crate::utils::traits::HasId;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicProductImage {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: BigDecimal,
}

impl HasId for PublicProductImage {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<ProductImageModel> for PublicProductImage {
    fn from(image: ProductImageModel) -> Self {
        Self {
            id: image.id,
            product_id: image.product_id,
            url: image.url,
            alt: image.alt,
            is_main: image.is_main,
            sort: image.sort,
        }
    }
}
