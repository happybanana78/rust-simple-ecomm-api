use crate::errors::error::AppError;
use crate::products::filters::ProductFilters;
use crate::products::images::dto::PublicProductImage;
use crate::products::model::ProductModel;
use crate::traits::{HasId, HasQuantity};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct PublicProduct {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
    pub images: Vec<PublicProductImage>,
}

impl HasId for PublicProduct {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl PublicProduct {
    pub fn from_model_with_images(product: ProductModel, images: Vec<PublicProductImage>) -> Self {
        Self {
            id: product.id,
            name: product.name,
            slug: product.slug,
            price: product.price,
            quantity: product.quantity,
            configurable: product.configurable,
            is_active: product.is_active,
            images,
        }
    }
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
            images: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct IndexProductDTO {
    #[validate(required, range(min = 1))]
    pub page: Option<i64>,

    #[validate(required, range(min = 1))]
    pub limit: Option<i64>,

    #[validate(length(min = 1))]
    pub search: Option<String>,

    #[validate(range(min = 1))]
    pub category: Option<i64>,

    #[validate(range(min = 0.0))]
    pub price_min: Option<f64>,

    #[validate(range(min = 0.0))]
    pub price_max: Option<f64>,
}

impl TryFrom<IndexProductDTO> for ProductFilters {
    type Error = AppError;

    fn try_from(dto: IndexProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            category: dto.category,
            price_min: dto.price_min,
            price_max: dto.price_max,
        })
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
