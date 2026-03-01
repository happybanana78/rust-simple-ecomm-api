use crate::app::cart::cart_items::dto::{AddItemCommand, RemoveItemCommand, UpdateItemCommand};
use crate::app::cart::cart_items::model::CartItemModel;
use crate::app::cart::cart_items::repository::CartItemsRepository;
use crate::app::products::repository::ProductRepository;
use crate::errors::error::AppError;
use crate::traits::IsRepository;
use sqlx::PgPool;

pub struct CartItemsService {
    repository: CartItemsRepository,
    product_repository: ProductRepository,
}

impl CartItemsService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: CartItemsRepository::new(pool.clone()),
            product_repository: ProductRepository::new(pool.clone()),
        }
    }

    pub async fn get_items(&self, cart_id: i64) -> Result<Vec<CartItemModel>, AppError> {
        self.repository.get_items(&cart_id).await
    }

    pub async fn add_item(&self, cmd: AddItemCommand) -> Result<(), AppError> {
        self.repository.add_item(cmd).await?;
        Ok(())
    }

    pub async fn remove_item(&self, cmd: RemoveItemCommand) -> Result<(), AppError> {
        let product_exist = self
            .repository
            .check_product_exist_in_cart(&cmd.product_id)
            .await?;

        if !product_exist {
            return Err(AppError::NotFound("product not found in cart".to_string()));
        }

        self.repository.remove_item(cmd).await?;
        Ok(())
    }

    pub async fn update_item(&self, cmd: UpdateItemCommand) -> Result<(), AppError> {
        let product_exist = self
            .repository
            .check_product_exist_in_cart(&cmd.product_id)
            .await?;

        if !product_exist {
            return Err(AppError::NotFound("product not found in cart".to_string()));
        }

        // TODO: handle configurable products

        let product_stock = self
            .product_repository
            .get_product_stock(&cmd.product_id)
            .await?;

        if cmd.quantity > product_stock {
            return Err(AppError::Internal("not enough stock available".to_string()));
        }

        self.repository.update_item(&cmd).await?;
        Ok(())
    }
}
