use crate::admin::products::filters::ProductFilters;
use crate::admin::products::model::AdminProductModel;
use crate::errors::error::AppError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminPublicProduct {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl From<AdminProductModel> for AdminPublicProduct {
    fn from(product: AdminProductModel) -> Self {
        Self {
            id: product.id,
            name: product.name,
            price: product.price,
            quantity: product.quantity,
            configurable: product.configurable,
            is_active: product.is_active,
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

    pub in_stock: Option<bool>,

    pub is_active: Option<bool>,
}

impl TryFrom<IndexProductDTO> for ProductFilters {
    type Error = AppError;

    fn try_from(dto: IndexProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            category: dto.category,
            price_min: dto.price_min,
            price_max: dto.price_max,
            in_stock: dto.in_stock,
            is_active: dto.is_active,
        })
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, length(min = 1))]
    pub slug: Option<String>,

    #[validate(length(min = 1))]
    pub categories: Option<Vec<i64>>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,

    #[validate(range(min = 0))]
    pub quantity: Option<i32>,

    pub configurable: Option<bool>,

    pub is_active: Option<bool>,
}

pub struct CreateProductCommand {
    pub name: String,
    pub slug: String,
    pub categories: Option<Vec<i64>>,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl TryFrom<CreateProductDTO> for CreateProductCommand {
    type Error = AppError;

    fn try_from(dto: CreateProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name.unwrap(),
            slug: dto.slug.unwrap(),
            categories: dto.categories,
            price: dto.price.unwrap(),
            quantity: dto.quantity.unwrap_or(0),
            configurable: dto.configurable.unwrap_or(false),
            is_active: dto.is_active.unwrap_or(true),
        })
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, length(min = 1))]
    pub slug: Option<String>,

    #[validate(length(min = 1))]
    pub categories: Option<Vec<i64>>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,

    #[validate(range(min = 0))]
    pub quantity: Option<i32>,

    pub configurable: Option<bool>,

    pub is_active: Option<bool>,
}

pub struct UpdateProductCommand {
    pub name: String,
    pub slug: String,
    pub categories: Option<Vec<i64>>,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl TryFrom<UpdateProductDTO> for UpdateProductCommand {
    type Error = AppError;

    fn try_from(dto: UpdateProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name.unwrap(),
            slug: dto.slug.unwrap(),
            categories: dto.categories,
            price: dto.price.unwrap(),
            quantity: dto.quantity.unwrap_or(0),
            configurable: dto.configurable.unwrap_or(false),
            is_active: dto.is_active.unwrap_or(true),
        })
    }
}
