use super::model::ProductModel;
use crate::errors::error::AppError;
use crate::products::dto::{PublicProduct, UpdateStockDto};
use crate::products::repository::ProductRepository;
use crate::products::traits::IntoPublic;
use sqlx::PgPool;

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(pool: PgPool) -> Self {
        let repository = ProductRepository::new(pool.clone());
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<ProductModel>, AppError> {
        self.repository.index().await
    }

    pub async fn get_all_public(&self) -> Result<Vec<PublicProduct>, AppError> {
        let products = self.get_all().await?;
        Ok(products.into_public())
    }

    pub async fn get_one(&self, id: &i64) -> Result<ProductModel, AppError> {
        let product = self.repository.show(id).await?;

        match product {
            Some(product) => Ok(product),
            None => Err(AppError::NotFound("Product not found".to_string())),
        }
    }

    pub async fn get_one_public(&self, id: &i64) -> Result<PublicProduct, AppError> {
        let product = self.get_one(id).await?;

        Ok(product.into_public())
    }

    pub async fn exist(&self, id: i64) -> Result<bool, AppError> {
        let product = self.repository.check_exist_and_active(id).await?;

        match product {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn update_stock(&self, dto: UpdateStockDto) -> Result<u64, AppError> {
        let product = self
            .repository
            .check_exist_and_active(dto.get_product_id())
            .await?;

        if product.is_none() {
            return Err(AppError::NotFound("Product not found".to_string()));
        }

        dto.check_quantity()?;

        self.repository
            .update_product_stock(dto.get_product_id(), dto.get_quantity())
            .await
    }
}
