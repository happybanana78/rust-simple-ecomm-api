use crate::cart::cart_items::model::CartItemModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct AddItemDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(required, range(min = 0.0))]
    pub price: Option<f64>,

    #[validate(required, range(min = 1))]
    pub quantity: Option<i32>,
}

pub struct AddItemCommand {
    pub product_id: i64,
    pub price: f64,
    pub quantity: i32,
    pub cart_id: i64,
}

impl AddItemCommand {
    pub fn new(dto: AddItemDto, cart_id: i64) -> Self {
        Self {
            product_id: dto.product_id.unwrap(),
            price: dto.price.unwrap(),
            quantity: dto.quantity.unwrap(),
            cart_id,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RemoveItemDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,
}

pub struct RemoveItemCommand {
    pub product_id: i64,
    pub cart_id: i64,
}

impl RemoveItemCommand {
    pub fn new(dto: RemoveItemDto, cart_id: i64) -> Self {
        Self {
            product_id: dto.product_id.unwrap(),
            cart_id,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateItemDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(required, range(min = 1))]
    pub quantity: Option<i32>,
}

pub struct UpdateItemCommand {
    pub product_id: i64,
    pub cart_id: i64,
    pub quantity: i32,
}

impl UpdateItemCommand {
    pub fn new(dto: UpdateItemDto, cart_id: i64) -> Self {
        Self {
            product_id: dto.product_id.unwrap(),
            quantity: dto.quantity.unwrap(),
            cart_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
