use super::model::AdminProductModel;
use crate::admin::categories::repository::AdminCategoryRepository;
use crate::admin::products::dto::{AdminPublicProduct, CreateProductCommand, UpdateProductCommand};
use crate::admin::products::filters::ProductFilters;
use crate::admin::products::repository::AdminProductRepository;
use crate::admin::products::traits::IntoPublic;
use crate::errors::error::AppError;
use crate::pagination::{DataCollection, Paginate, PaginatedDataCollection};
use crate::traits::IsRepository;
use crate::validation_utils::validate_slug;
use sqlx::PgPool;

pub struct AdminProductService {
    repository: AdminProductRepository,
    category_repository: AdminCategoryRepository,
}

impl AdminProductService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminProductRepository::new(pool.clone()),
            category_repository: AdminCategoryRepository::new(pool),
        }
    }

    pub async fn get_all(&self) -> Result<DataCollection<AdminProductModel>, AppError> {
        let data = self.repository.index().await?;
        Ok(DataCollection::new(data))
    }

    pub async fn get_all_paginated(
        &self,
        pagination: &Paginate,
        filters: &ProductFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<AdminProductModel>, AppError> {
        let data = self
            .repository
            .index_paginated(pagination, search, filters)
            .await?;
        Ok(PaginatedDataCollection::new(data, pagination.clone()))
    }

    pub async fn get_all_public(&self) -> Result<DataCollection<AdminPublicProduct>, AppError> {
        let data = self.get_all().await?;
        Ok(data.into_public())
    }

    pub async fn get_all_paginated_public(
        &self,
        pagination: &Paginate,
        filters: &ProductFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<AdminPublicProduct>, AppError> {
        let data = self.get_all_paginated(pagination, filters, search).await?;
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
        validate_slug(&cmd.slug)?;

        let product_already_exists = self.check_exist_with_same_slug(&cmd.slug).await?;

        if product_already_exists {
            return Err(AppError::Conflict(
                "Product with the same slug already exists".to_string(),
            ));
        }

        let mut tx = self.repository.start_transaction().await?;

        let product = self.repository.create(&mut *tx, &cmd).await?;

        if let Some(categories) = &cmd.categories {
            for category_id in categories {
                let category = self
                    .category_repository
                    .check_existence_by_id(*category_id)
                    .await?;

                if !category {
                    return Err(AppError::NotFound(format!(
                        "Category with id {} not found",
                        category_id
                    )));
                }

                self.repository
                    .attach_product_to_category(&mut *tx, product.id, *category_id)
                    .await?;
            }
        }

        self.repository.commit_transaction(tx).await?;

        Ok(product)
    }

    pub async fn update(&self, cmd: UpdateProductCommand, id: i64) -> Result<(), AppError> {
        validate_slug(&cmd.slug)?;

        self.get_one(id).await?;

        let product_already_exists = self.check_exist_with_same_slug(&cmd.slug).await?;

        if product_already_exists {
            return Err(AppError::Conflict(
                "Product with the same slug already exists".to_string(),
            ));
        }

        let mut tx = self.repository.start_transaction().await?;

        if let Some(categories) = &cmd.categories {
            self.repository
                .detach_product_from_all_categories(&mut *tx, id)
                .await?;

            for category_id in categories {
                let category = self
                    .category_repository
                    .check_existence_by_id(*category_id)
                    .await?;

                if !category {
                    return Err(AppError::NotFound(format!(
                        "Category with id {} not found",
                        category_id
                    )));
                }

                self.repository
                    .attach_product_to_category(&mut *tx, id, *category_id)
                    .await?;
            }
        }

        self.repository.update(&mut *tx, cmd, id).await?;

        self.repository.commit_transaction(tx).await
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        self.get_one(id).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }

    pub async fn check_exist_with_same_slug(&self, name: &str) -> Result<bool, AppError> {
        self.repository.check_existence_by_slug(name).await
    }
}
