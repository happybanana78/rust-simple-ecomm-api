use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository as cart_items_repository;
use crate::cart::guest_cart::dto::PublicGuestCart;
use crate::cart::guest_cart::repository;
use crate::errors::error::AppError;
use crate::users::service as users_service;
use sqlx::PgPool;

/**
* Returns cart for guest user
*/
pub async fn get_cart_by_hash(pool: &PgPool, hash: &str) -> Result<PublicGuestCart, AppError> {
    let user_hash = users_service::get_user_hash(pool, hash)
        .await?
        .ok_or_else(|| AppError::NotFound("user hash not found".to_string()))?;

    let cart = repository::get_cart_by_user_hash(pool, &user_hash.id).await?;

    let cart = match cart {
        Some(cart) => cart,
        None => {
            let cart = repository::create_hash_cart(pool, &user_hash.id).await?;
            return Ok(PublicGuestCart::new_from_model(cart));
        }
    };

    let cart_items: Vec<CartItemModel> = cart_items_repository::get_items(&pool, &cart.id).await?;

    Ok(PublicGuestCart::new_with_items(cart, cart_items))
}

pub async fn get_cart_id_by_hash(pool: &PgPool, hash: &str) -> Result<i64, AppError> {
    let user_hash = users_service::get_user_hash(pool, hash)
        .await?
        .ok_or_else(|| AppError::NotFound("user hash not found".to_string()))?;

    let cart_id = repository::get_cart_id(pool, &user_hash.id).await?;

    match cart_id {
        Some(cart_id) => Ok(cart_id.id),
        None => {
            let cart = repository::create_hash_cart(pool, &user_hash.id).await?;
            Ok(cart.id)
        }
    }
}
