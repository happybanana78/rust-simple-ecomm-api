use crate::errors::error::AppError;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,
}

pub struct CreateProductCommand {
    pub name: String,
    pub price: f64,
}

impl TryFrom<CreateProductDTO> for CreateProductCommand {
    type Error = AppError;

    fn try_from(dto: CreateProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name.unwrap(),
            price: dto.price.unwrap(),
        })
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,
}

pub struct UpdateProductCommand {
    pub name: String,
    pub price: f64,
}

impl TryFrom<UpdateProductDTO> for UpdateProductCommand {
    type Error = AppError;

    fn try_from(dto: UpdateProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name.unwrap(),
            price: dto.price.unwrap(),
        })
    }
}
