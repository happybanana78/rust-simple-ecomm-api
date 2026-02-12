use crate::admin::products::filters::ProductFilters;
use crate::admin::products::model::AdminProductModel;
use crate::errors::error::AppError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
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

#[derive(Deserialize, Validate)]
pub struct IndexProductDTO {
    #[validate(required, range(min = 1))]
    pub page: Option<i64>,

    #[validate(required, range(min = 1))]
    pub limit: Option<i64>,

    #[validate(length(min = 1))]
    pub search: Option<String>,

    #[validate(nested)]
    pub filters: Option<ProductFilters>,
}

#[derive(Deserialize, Validate)]
pub struct CreateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,

    #[validate(range(min = 0))]
    pub quantity: Option<i32>,

    pub configurable: Option<bool>,

    pub is_active: Option<bool>,
}

pub struct CreateProductCommand {
    pub name: String,
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
            price: dto.price.unwrap(),
            quantity: dto.quantity.unwrap_or(0),
            configurable: dto.configurable.unwrap_or(false),
            is_active: dto.is_active.unwrap_or(true),
        })
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,

    #[validate(range(min = 0))]
    pub quantity: Option<i32>,

    pub configurable: Option<bool>,

    pub is_active: Option<bool>,
}

pub struct UpdateProductCommand {
    pub name: String,
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
            price: dto.price.unwrap(),
            quantity: dto.quantity.unwrap_or(0),
            configurable: dto.configurable.unwrap_or(false),
            is_active: dto.is_active.unwrap_or(true),
        })
    }
}
