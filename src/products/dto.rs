use crate::errors::error::AppError;
use crate::products::model::ProductModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicProduct {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,
}

impl From<ProductModel> for PublicProduct {
    fn from(product: ProductModel) -> Self {
        Self {
            id: product.id,
            name: product.name,
            price: product.price,
            quantity: product.quantity,
            configurable: product.configurable,
            is_active: product.is_active,
        }
    }
}

pub struct UpdateStockDto {
    product_id: i64,
    quantity: i32,
}

impl UpdateStockDto {
    pub fn new(product_id: i64, quantity: i32) -> Self {
        Self {
            product_id,
            quantity,
        }
    }

    pub fn get_product_id(&self) -> i64 {
        self.product_id
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn check_quantity(&self) -> Result<(), AppError> {
        if self.quantity < 0 {
            return Err(AppError::Internal(
                "Quantity cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}
