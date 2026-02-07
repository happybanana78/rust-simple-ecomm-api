use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository as cart_items_repository;
use crate::cart::user_cart::dto::PublicUserCart;
use crate::cart::user_cart::repository;
use crate::errors::error::AppError;
use crate::users::service as users_service;
use sqlx::PgPool;

/**
* Returns cart for authenticated user
*/
pub async fn get_cart_by_user(pool: &PgPool, user_id: &i64) -> Result<PublicUserCart, AppError> {
    let user = users_service::get_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

    let cart = repository::get_cart_by_user_id(pool, &user.id).await?;

    let cart = match cart {
        Some(cart) => cart,
        None => {
            let cart = repository::create_user_cart(pool, &user.id).await?;
            return Ok(PublicUserCart::new_from_model(cart));
        }
    };

    let cart_items: Vec<CartItemModel> = cart_items_repository::get_items(pool, &cart.id).await?;

    Ok(PublicUserCart::new_with_items(cart, cart_items))
}

pub async fn get_cart_id_by_user(pool: &PgPool, user_id: &i64) -> Result<i64, AppError> {
    let user = users_service::get_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

    let cart_id = repository::get_cart_id(pool, &user.id).await?;

    match cart_id {
        Some(cart_id) => Ok(cart_id.id),
        None => {
            let cart = repository::create_user_cart(pool, &user.id).await?;
            Ok(cart.id)
        }
    }
}
