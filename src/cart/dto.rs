use crate::cart::cart_items::dto::PublicCartItems;
use crate::cart::cart_items::model::CartItemModel;
use crate::cart::model::{HashCartModel, UserCartModel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
#[validate(schema(function = "check_validity"))]
pub struct GetCartDto {
    pub user_id: Option<i64>,
    pub user_hash: Option<String>,
}

/**
* One of the two fields must be present
*/
fn check_validity(dto: &GetCartDto) -> Result<(), ValidationError> {
    match (&dto.user_id, &dto.user_hash) {
        (None, None) => Err(ValidationError::new("user_id_or_hash_required")),
        _ => Ok(()),
    }
}

pub struct CartByUserCommand {
    pub user_id: i64,
}

pub struct CartByHashCommand {
    pub user_hash: String,
}

pub enum GetCartCommandEnum {
    ByUser(CartByUserCommand),
    ByHash(CartByHashCommand),
}

impl GetCartDto {
    pub fn into_command(self) -> GetCartCommandEnum {
        match (self.user_id, self.user_hash) {
            (Some(user_id), None) => GetCartCommandEnum::ByUser(CartByUserCommand { user_id }),
            (None, Some(user_hash)) => GetCartCommandEnum::ByHash(CartByHashCommand { user_hash }),
            (Some(user_id), Some(_)) => GetCartCommandEnum::ByUser(CartByUserCommand { user_id }),
            (None, None) => unreachable!("DTO already validated"),
        }
    }
}

#[derive(Serialize)]
pub struct PublicUserCart {
    pub id: i64,
    pub user_id: i64,
    pub total: f64,
    pub created_at: DateTime<Utc>,
    pub items: Vec<PublicCartItems>,
}

impl PublicUserCart {
    pub fn new_from_model(cart: UserCartModel) -> Self {
        PublicUserCart {
            id: cart.id,
            user_id: cart.user_id,
            total: cart.total,
            created_at: cart.created_at,
            items: Vec::new(),
        }
    }

    pub fn new_with_items(cart: UserCartModel, items: Vec<CartItemModel>) -> Self {
        PublicUserCart {
            id: cart.id,
            user_id: cart.user_id,
            total: cart.total,
            created_at: cart.created_at,
            items: items
                .into_iter()
                .map(|item| PublicCartItems::from(item))
                .collect(),
        }
    }
}

#[derive(Serialize)]
pub struct PublicHashCart {
    pub id: i64,
    pub user_hash_id: i64,
    pub total: f64,
    pub created_at: DateTime<Utc>,
    pub items: Vec<PublicCartItems>,
}

impl PublicHashCart {
    pub fn new_from_model(cart: HashCartModel) -> Self {
        PublicHashCart {
            id: cart.id,
            user_hash_id: cart.user_hash_id,
            total: cart.total,
            created_at: cart.created_at,
            items: Vec::new(),
        }
    }

    pub fn new_with_items(cart: HashCartModel, items: Vec<CartItemModel>) -> Self {
        PublicHashCart {
            id: cart.id,
            user_hash_id: cart.user_hash_id,
            total: cart.total,
            created_at: cart.created_at,
            items: items
                .into_iter()
                .map(|item| PublicCartItems::from(item))
                .collect(),
        }
    }
}
