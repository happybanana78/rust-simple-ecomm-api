use crate::app::cart::cart_items::dto::PublicCartItems;
use crate::app::cart::cart_items::model::CartItemModel;
use crate::app::cart::user_cart::model::UserCartModel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
            items: items.into_iter().map(PublicCartItems::from).collect(),
        }
    }
}
