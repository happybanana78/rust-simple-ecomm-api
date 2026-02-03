use sqlx::PgPool;
use crate::cart::model::CartModel;
use crate::cart::repository;
use crate::users::service as users_service;
use crate::errors::error::AppError;

pub async fn get_cart_by_user(pool: &PgPool, user_id: i64) -> Result<CartModel, AppError> {
    let user = users_service::get_user_by_id(pool, user_id).await?;

    if user.is_none() {
        return Err(AppError::NotFound("user not found".to_string()))
    }

    let cart = repository::get_cart_by_user_id(pool, user_id).await?;

    if cart.is_none() {
        return create_cart(pool, &user_id).await;
    }

    // TODO: get cart items as well

    Ok(cart.unwrap())
}

pub async fn get_cart_by_hash(pool: &PgPool, hash: String) -> Result<CartModel, AppError> {
    let user = users_service::get_user_by_id(pool, user_id).await?;

    if user.is_none() {
        return Err(AppError::NotFound("user not found".to_string()))
    }

    let cart = repository::get_cart_by_user_id(pool, user_id).await?;

    if cart.is_none() {
        return create_cart(pool, &user_id).await;
    }

    // TODO: get cart items as well

    Ok(cart.unwrap())
}

async fn create_cart(pool: &PgPool, user_id: &i64) -> Result<CartModel, AppError> {
    repository::create_cart(pool, user_id).await
}
