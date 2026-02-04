use crate::cart::cart_items::model::CartItemModel;
use crate::errors::error::AppError;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AddItemDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,

    #[validate(required, range(min = 1))]
    pub quantity: Option<i32>,

    #[validate(required, range(min = 1))]
    pub cart_id: Option<i64>,
}

pub struct AddItemCommand {
    pub product_id: i64,
    pub price: f64,
    pub quantity: i32,
    pub cart_id: i64,
}

impl TryFrom<AddItemDto> for AddItemCommand {
    type Error = AppError;

    fn try_from(value: AddItemDto) -> Result<Self, Self::Error> {
        Ok(Self {
            product_id: value.product_id.unwrap(),
            price: value.price.unwrap(),
            quantity: value.quantity.unwrap(),
            cart_id: value.cart_id.unwrap(),
        })
    }
}

#[derive(Deserialize, Validate)]
pub struct RemoveItemDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(required, range(min = 1))]
    pub cart_id: Option<i64>,
}

pub struct RemoveItemCommand {
    pub product_id: i64,
    pub cart_id: i64,
}

impl TryFrom<RemoveItemDto> for RemoveItemCommand {
    type Error = AppError;

    fn try_from(value: RemoveItemDto) -> Result<Self, Self::Error> {
        Ok(Self {
            product_id: value.product_id.unwrap(),
            cart_id: value.cart_id.unwrap(),
        })
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateItemDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(required, range(min = 1))]
    pub cart_id: Option<i64>,

    #[validate(required, range(min = 1))]
    pub quantity: Option<i32>,
}

pub struct UpdateItemCommand {
    pub product_id: i64,
    pub cart_id: i64,
    pub quantity: i32,
}

impl TryFrom<UpdateItemDto> for UpdateItemCommand {
    type Error = AppError;

    fn try_from(value: UpdateItemDto) -> Result<Self, Self::Error> {
        Ok(Self {
            product_id: value.product_id.unwrap(),
            quantity: value.quantity.unwrap(),
            cart_id: value.cart_id.unwrap(),
        })
    }
}

#[derive(Serialize)]
pub struct PublicCartItems {
    pub id: i64,
    pub cart_id: i64,
    pub product_id: i64,
    pub price: f64,
    pub quantity: i32,
}

impl From<CartItemModel> for PublicCartItems {
    fn from(item: CartItemModel) -> Self {
        PublicCartItems {
            id: item.id,
            cart_id: item.cart_id,
            product_id: item.product_id,
            price: item.price,
            quantity: item.quantity,
        }
    }
}
