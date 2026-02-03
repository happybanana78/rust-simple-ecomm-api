use sqlx::PgPool;
use crate::cart::model::{HashCartModel, UserCartModel};
use crate::cart::repository;
use crate::users::service as users_service;
use crate::errors::error::AppError;

pub async fn get_cart_by_user(pool: &PgPool, user_id: i64) -> Result<UserCartModel, AppError> {
    let user = users_service::get_user_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

    let cart = repository::get_cart_by_user_id(pool, &user.id).await?;

    if cart.is_none() {
        return repository::create_user_cart(pool, &user.id).await
    }

    // TODO: get cart items as well
    // TODO: add authentication check

    Ok(cart.unwrap())
}

pub async fn get_cart_by_hash(pool: &PgPool, hash: String) -> Result<HashCartModel, AppError> {
    let user_hash = users_service::get_user_hash(pool, hash.as_str())
        .await?
        .ok_or_else(|| AppError::NotFound("user hash not found".to_string()))?;

    let cart = repository::get_cart_by_user_hash(pool, &user_hash.id).await?;

    if cart.is_none() {
        return repository::create_hash_cart(pool, &user_hash.id).await
    }

    // TODO: get cart items as well

    Ok(cart.unwrap())
}
