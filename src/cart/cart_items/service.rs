use crate::cart::cart_items::dto::{AddItemCommand, RemoveItemCommand, UpdateItemCommand};
use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository;
use crate::errors::error::AppError;
use sqlx::PgPool;

pub async fn get_items(pool: &PgPool, cart_id: i64) -> Result<Vec<CartItemModel>, AppError> {
    repository::get_items(pool, &cart_id).await
}

pub async fn add_item(pool: &PgPool, cmd: AddItemCommand) -> Result<(), AppError> {
    repository::add_item(pool, cmd).await?;
    Ok(())
}

pub async fn remove_item(pool: &PgPool, cmd: RemoveItemCommand) -> Result<(), AppError> {
    let product_exist = repository::check_product_exist_in_cart(pool, &cmd.product_id).await?;

    if !product_exist {
        return Err(AppError::NotFound("product not found in cart".to_string()));
    }

    repository::remove_item(pool, cmd).await?;
    Ok(())
}

pub async fn update_item(pool: &PgPool, cmd: UpdateItemCommand) -> Result<(), AppError> {
    let product_exist = repository::check_product_exist_in_cart(pool, &cmd.product_id).await?;

    if !product_exist {
        return Err(AppError::NotFound("product not found in cart".to_string()));
    }

    // TODO: check product stock

    repository::update_item(pool, cmd).await?;
    Ok(())
}
