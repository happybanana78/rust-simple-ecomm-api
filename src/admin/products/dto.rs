use crate::errors::error::AppError;
use serde::Deserialize;
use validator::Validate;

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
