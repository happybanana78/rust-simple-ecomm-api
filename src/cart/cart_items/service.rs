use sqlx::PgPool;
use crate::cart::cart_items::dto::{AddItemCommand, RemoveItemCommand, UpdateItemCommand};
use crate::cart::cart_items::model::CartItemModel;
use crate::cart::cart_items::repository;
use crate::errors::error::AppError;

pub async fn get_items(pool: &PgPool, cart_id: i64) -> Result<Vec<CartItemModel>, AppError> {
    repository::get_items(pool, &cart_id).await
}

pub async fn add_item(pool: &PgPool, cmd: AddItemCommand) -> Result<(), AppError> {
    repository::add_item(pool, cmd).await?;
    Ok(())
}

pub async fn remove_item(pool: &PgPool, cmd: RemoveItemCommand) -> Result<(), AppError> {
    repository::remove_item(pool, cmd).await?;
    Ok(())
}

pub async fn update_item(pool: &PgPool, cmd: UpdateItemCommand) -> Result<(), AppError> {
    repository::update_item(pool, cmd).await?;
    Ok(())
}
