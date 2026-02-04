use sqlx::PgPool;
use crate::cart::cart_items::model::CartItemModel;
use crate::cart::dto::{PublicHashCart, PublicUserCart};
use crate::cart::repository;
use crate::cart::cart_items::repository as cart_items_repository;
use crate::users::service as users_service;
use crate::errors::error::AppError;

/**
* Returns cart for authenticated user
*/
pub async fn get_cart_by_user(pool: &PgPool, user_id: i64) -> Result<PublicUserCart, AppError> {
    let user = users_service::get_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

    let cart = repository::get_cart_by_user_id(pool, &user.id).await?;

    let cart = match cart {
        Some(cart) => cart,
        None => {
            let cart = repository::create_user_cart(pool, &user.id).await?;
            return Ok(PublicUserCart::new_from_model(cart))
        }
    };

    let cart_items: Vec<CartItemModel> = cart_items_repository::get_items(&pool, &cart.id).await?;

    // TODO: add authentication check

    Ok(PublicUserCart::new_with_items(cart, cart_items))
}

/**
* Returns cart for guest user
*/
pub async fn get_cart_by_hash(pool: &PgPool, hash: String) -> Result<PublicHashCart, AppError> {
    let user_hash = users_service::get_user_hash(pool, hash.as_str())
        .await?
        .ok_or_else(|| AppError::NotFound("user hash not found".to_string()))?;

    let cart = repository::get_cart_by_user_hash(pool, &user_hash.id).await?;

    let cart = match cart {
        Some(cart) => cart,
        None => {
            let cart = repository::create_hash_cart(pool, &user_hash.id).await?;
            return Ok(PublicHashCart::new_from_model(cart))
        }
    };

    let cart_items: Vec<CartItemModel> = cart_items_repository::get_items(&pool, &cart.id).await?;

    Ok(PublicHashCart::new_with_items(cart, cart_items))
}
