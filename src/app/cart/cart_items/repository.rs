use crate::app::cart::cart_items::dto::{AddItemCommand, RemoveItemCommand, UpdateItemCommand};
use crate::app::cart::cart_items::model::CartItemModel;
use crate::errors::error::AppError;
use crate::traits::IsRepository;
use sqlx::PgPool;

pub struct CartItemsRepository {
    pool: PgPool,
}

impl IsRepository for CartItemsRepository {
    type Repository = Self;

    fn new(pool: PgPool) -> Self::Repository {
        Self { pool }
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl CartItemsRepository {
    pub async fn get_items(&self, cart_id: &i64) -> Result<Vec<CartItemModel>, AppError> {
        sqlx::query_as!(
        CartItemModel,
        "SELECT id, cart_id, product_id, price, quantity, created_at FROM cart_items WHERE cart_id = $1;",
        cart_id
    )
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)
    }

    pub async fn add_item(&self, cmd: AddItemCommand) -> Result<u64, AppError> {
        let result = sqlx::query_as!(
            CartItemModel,
            r#"INSERT INTO cart_items (cart_id, product_id, price, quantity, created_at)
            VALUES ($1, $2, $3, $4, NOW());"#,
            cmd.cart_id,
            cmd.product_id,
            cmd.price,
            cmd.quantity
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn remove_item(&self, cmd: RemoveItemCommand) -> Result<u64, AppError> {
        let result = sqlx::query_as!(
            CartItemModel,
            "DELETE FROM cart_items WHERE cart_id = $1 AND product_id = $2;",
            cmd.cart_id,
            cmd.product_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn update_item(&self, cmd: &UpdateItemCommand) -> Result<u64, AppError> {
        let result = sqlx::query_as!(
            CartItemModel,
            "UPDATE cart_items SET quantity = $1 WHERE cart_id = $2 AND product_id = $3;",
            cmd.quantity,
            cmd.cart_id,
            cmd.product_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.rows_affected())
    }

    pub async fn check_product_exist_in_cart(&self, product_id: &i64) -> Result<bool, AppError> {
        sqlx::query_scalar!(
            r#"SELECT EXISTS(
            SELECT 1 FROM cart_items WHERE product_id = $1
        )
        as "exists!";
        "#,
            product_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }

    pub async fn get_cart_product_quantity(
        &self,
        cart_id: &i64,
        product_id: &i64,
    ) -> Result<i32, AppError> {
        sqlx::query_scalar!(
            r#"
        SELECT
            quantity
        FROM cart_items
        WHERE cart_id = $1 AND product_id = $2;
        "#,
            cart_id,
            product_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
    }
}
