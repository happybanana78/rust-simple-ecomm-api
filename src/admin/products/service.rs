use super::model::AdminProductModel;
use crate::admin::products::dto::{AdminPublicProduct, CreateProductCommand, UpdateProductCommand};
use crate::admin::products::repository::AdminProductRepository;
use crate::admin::products::traits::IntoPublic;
use crate::errors::error::AppError;
use crate::pagination::{DataCollection, Paginate, PaginatedDataCollection};
use sqlx::PgPool;

pub struct AdminProductService {
    repository: AdminProductRepository,
}

impl AdminProductService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminProductRepository::new(pool),
        }
    }

    pub async fn get_all(&self) -> Result<DataCollection<AdminProductModel>, AppError> {
        let data = self.repository.index().await?;
        Ok(DataCollection::new(data))
    }

    pub async fn get_all_paginated(
        &self,
        pagination: Paginate,
    ) -> Result<PaginatedDataCollection<AdminProductModel>, AppError> {
        let data = self.repository.index_paginated(&pagination).await?;
        Ok(PaginatedDataCollection::new(data, pagination))
    }

    pub async fn get_all_public(&self) -> Result<DataCollection<AdminPublicProduct>, AppError> {
        let data = self.get_all().await?;
        Ok(data.into_public())
    }

    pub async fn get_all_paginated_public(
        &self,
        pagination: Paginate,
    ) -> Result<PaginatedDataCollection<AdminPublicProduct>, AppError> {
        let data = self.get_all_paginated(pagination).await?;
        Ok(data.into_public())
    }

    pub async fn get_one(&self, id: i64) -> Result<AdminProductModel, AppError> {
        let product = self.repository.show(id).await?;

        match product {
            Some(product) => Ok(product),
            None => Err(AppError::NotFound("Product not found".to_string())),
        }
    }

    pub async fn get_one_public(&self, id: i64) -> Result<AdminPublicProduct, AppError> {
        let product = self.get_one(id).await?;

        Ok(product.into_public())
    }

    pub async fn create(&self, cmd: CreateProductCommand) -> Result<AdminProductModel, AppError> {
        self.repository.create(cmd).await
    }

    pub async fn update(&self, cmd: UpdateProductCommand, id: i64) -> Result<u64, AppError> {
        self.repository.update(cmd, id).await
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        self.repository.delete(id).await
    }
}
