use crate::app::cart::cart_items::dto::PublicCartItems;
use crate::app::cart::cart_items::model::CartItemModel;
use crate::app::cart::guest_cart::model::GuestCartModel;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicGuestCart {
    pub id: i64,
    pub user_hash_id: i64,
    pub total: f64,
    pub created_at: DateTime<Utc>,
    pub items: Vec<PublicCartItems>,
}

impl PublicGuestCart {
    pub fn new_from_model(cart: GuestCartModel) -> Self {
        PublicGuestCart {
            id: cart.id,
            user_hash_id: cart.user_hash_id,
            total: cart.total,
            created_at: cart.created_at,
            items: Vec::new(),
        }
    }

    pub fn new_with_items(cart: GuestCartModel, items: Vec<CartItemModel>) -> Self {
        PublicGuestCart {
            id: cart.id,
            user_hash_id: cart.user_hash_id,
            total: cart.total,
            created_at: cart.created_at,
            items: items.into_iter().map(PublicCartItems::from).collect(),
        }
    }
}
