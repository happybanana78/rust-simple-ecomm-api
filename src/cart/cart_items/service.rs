use crate::cart::cart_items::dto::{AddItemCommand, RemoveItemCommand, UpdateItemCommand};
use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository as cart_items_repository;
use crate::errors::error::AppError;
use crate::products::repository as product_repository;
use sqlx::PgPool;

pub async fn get_items(pool: &PgPool, cart_id: i64) -> Result<Vec<CartItemModel>, AppError> {
    cart_items_repository::get_items(pool, &cart_id).await
}

pub async fn add_item(pool: &PgPool, cmd: AddItemCommand) -> Result<(), AppError> {
    cart_items_repository::add_item(pool, cmd).await?;
    Ok(())
}

pub async fn remove_item(pool: &PgPool, cmd: RemoveItemCommand) -> Result<(), AppError> {
    let product_exist =
        cart_items_repository::check_product_exist_in_cart(pool, &cmd.product_id).await?;

    if !product_exist {
        return Err(AppError::NotFound("product not found in cart".to_string()));
    }

    cart_items_repository::remove_item(pool, cmd).await?;
    Ok(())
}

pub async fn update_item(pool: &PgPool, cmd: UpdateItemCommand) -> Result<(), AppError> {
    let product_exist =
        cart_items_repository::check_product_exist_in_cart(pool, &cmd.product_id).await?;

    if !product_exist {
        return Err(AppError::NotFound("product not found in cart".to_string()));
    }

    // TODO: handle configurable products

    let product_stock = product_repository::get_product_stock(pool, &cmd.product_id).await?;

    if cmd.quantity > product_stock {
        return Err(AppError::Internal("not enough stock available".to_string()));
    }

    cart_items_repository::update_item(pool, &cmd).await?;
    Ok(())
}
