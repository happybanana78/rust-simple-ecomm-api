use super::model::ProductModel;
use crate::errors::error::AppError;
use crate::pagination::{Paginate, PaginatedDataCollection};
use crate::products::dto::{PublicProduct, UpdateStockDto};
use crate::products::filters::ProductFilters;
use crate::products::images::repository::ProductImageRepository;
use crate::products::images::traits::IntoPublic as IntoPublicProductImage;
use crate::products::repository::ProductRepository;
use crate::products::traits::IntoPublic;
use crate::traits::{HasQuantity, IsRepository};
use sqlx::PgPool;

pub struct ProductService {
    repository: ProductRepository,
    product_image_repository: ProductImageRepository,
}

impl ProductService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: ProductRepository::new(pool.clone()),
            product_image_repository: ProductImageRepository::new(pool),
        }
    }

    pub async fn get_all_paginated(
        &self,
        pagination: &Paginate,
        filters: &ProductFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<ProductModel>, AppError> {
        let data = self
            .repository
            .index_paginated(pagination, search, filters)
            .await?;

        Ok(PaginatedDataCollection::new(data, pagination.clone()))
    }

    /**
     * Get all paginated products with images (public version).
     */
    pub async fn get_all_paginated_public(
        &self,
        pagination: &Paginate,
        filters: &ProductFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<PublicProduct>, AppError> {
        let products = self.get_all_paginated(pagination, filters, search).await?;
        let images = self
            .product_image_repository
            .get_all_for_multiple_products(products.extract_ids())
            .await?
            .into_public();

        Ok(products.into_public_with_images(images))
    }

    pub async fn get_one(&self, slug: &str) -> Result<ProductModel, AppError> {
        let product = self.repository.show(slug).await?;

        match product {
            Some(product) => Ok(product),
            None => Err(AppError::NotFound("Product not found".to_string())),
        }
    }

    pub async fn get_one_public(&self, slug: &str) -> Result<PublicProduct, AppError> {
        let product = self.get_one(slug).await?;
        let images = self
            .product_image_repository
            .get_all_by_product(*product.id)
            .await?
            .into_public();

        Ok(product.into_public_with_images(images))
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
            .check_exist_and_active(dto.product_id)
            .await?;

        if product.is_none() {
            return Err(AppError::NotFound("Product not found".to_string()));
        }

        if !dto.is_safe_quantity() {
            return Err(AppError::Internal(
                "Quantity cannot be negative".to_string(),
            ));
        }

        self.repository
            .update_product_stock(dto.product_id, dto.quantity)
            .await
    }
}
