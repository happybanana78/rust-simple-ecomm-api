use super::model::ProductModel;
use crate::app::products::dto::{PublicProduct, PublicProductBuilder, UpdateStockDto};
use crate::app::products::filters::ProductFilters;
use crate::app::products::images::dto::PublicProductImage;
use crate::app::products::images::repository::ProductImageRepository;
use crate::app::products::images::traits::IntoPublic as IntoPublicProductImage;
use crate::app::products::relations::{ProductLoadRelations, ProductRelations};
use crate::app::products::repository::ProductRepository;
use crate::app::products::reviews::dto::PublicProductReview;
use crate::app::products::reviews::repository::ProductReviewRepository;
use crate::app::products::reviews::traits::IntoPublic as IntoPublicProductReview;
use crate::app::products::videos::dto::PublicProductVideo;
use crate::app::products::videos::repository::ProductVideoRepository;
use crate::app::products::videos::traits::IntoPublic as IntoPublicProductVideo;
use crate::errors::error::AppError;
use crate::pagination::{Paginate, PaginatedDataCollection};
use crate::traits::{HasQuantity, IsRepository};
use futures_util::future::try_join_all;
use futures_util::{FutureExt, TryFutureExt};
use sqlx::PgPool;
use std::collections::HashMap;

pub struct ProductService {
    repository: ProductRepository,
    product_image_repository: ProductImageRepository,
    product_video_repository: ProductVideoRepository,
    product_review_repository: ProductReviewRepository,
}

impl ProductService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: ProductRepository::new(pool.clone()),
            product_image_repository: ProductImageRepository::new(pool.clone()),
            product_video_repository: ProductVideoRepository::new(pool.clone()),
            product_review_repository: ProductReviewRepository::new(pool),
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
        relations: ProductLoadRelations,
    ) -> Result<PaginatedDataCollection<PublicProduct>, AppError> {
        let product_collection = self.get_all_paginated(pagination, filters, search).await?;

        let loaded_relations = self
            .load_relations(relations, product_collection.extract_ids())
            .await?;

        let mut images_by_product_id: HashMap<i64, Vec<PublicProductImage>> = HashMap::new();
        let mut videos_by_product_id: HashMap<i64, Vec<PublicProductVideo>> = HashMap::new();
        let mut reviews_by_product_id: HashMap<i64, Vec<PublicProductReview>> = HashMap::new();

        for relation in loaded_relations {
            match relation {
                ProductRelations::Images(images) => {
                    for image in images {
                        images_by_product_id
                            .entry(image.product_id)
                            .or_default()
                            .push(image);
                    }
                }
                ProductRelations::Videos(videos) => {
                    for video in videos {
                        videos_by_product_id
                            .entry(video.product_id)
                            .or_default()
                            .push(video);
                    }
                }
                ProductRelations::Reviews(reviews) => {
                    for review in reviews {
                        reviews_by_product_id
                            .entry(review.product_id)
                            .or_default()
                            .push(review);
                    }
                }
            }
        }

        let mut public_products = Vec::new();

        for product in product_collection.get_data() {
            let mut product_builder = PublicProductBuilder::new(&product);

            if let Some(images) = images_by_product_id.remove(&product.id) {
                product_builder = product_builder.with_images(images);
            }

            if let Some(videos) = videos_by_product_id.remove(&product.id) {
                product_builder = product_builder.with_videos(videos);
            }

            if let Some(reviews) = reviews_by_product_id.remove(&product.id) {
                product_builder = product_builder.with_reviews(reviews);
            }

            public_products.push(product_builder.build())
        }

        Ok(PaginatedDataCollection::new(
            public_products,
            product_collection.get_pagination(),
        ))
    }

    pub async fn get_one(&self, slug: &str) -> Result<ProductModel, AppError> {
        let product = self.repository.show(slug).await?;

        match product {
            Some(product) => Ok(product),
            None => Err(AppError::NotFound("Product not found".to_string())),
        }
    }

    pub async fn get_one_public(
        &self,
        slug: &str,
        relations: ProductLoadRelations,
    ) -> Result<PublicProduct, AppError> {
        let product = self.get_one(slug).await?;

        let loaded_relations = self.load_relations(relations, vec![product.id]).await?;

        let mut product_builder = PublicProductBuilder::new(&product);

        for relation in loaded_relations {
            match relation {
                ProductRelations::Images(images) => {
                    product_builder = product_builder.with_images(images);
                }
                ProductRelations::Videos(videos) => {
                    product_builder = product_builder.with_videos(videos);
                }
                ProductRelations::Reviews(reviews) => {
                    product_builder = product_builder.with_reviews(reviews);
                }
            }
        }

        Ok(product_builder.build())
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

    async fn load_relations(
        &self,
        relations: ProductLoadRelations,
        product_ids: Vec<i64>,
    ) -> Result<Vec<ProductRelations>, AppError> {
        let mut futures = Vec::new();

        if relations.images {
            futures.push(
                self.product_image_repository
                    .get_all_for_multiple_products(&product_ids)
                    .map_ok(|data| ProductRelations::Images(data.into_public()))
                    .boxed(),
            );
        }

        if relations.videos {
            futures.push(
                self.product_video_repository
                    .get_all_for_multiple_products(&product_ids)
                    .map_ok(|data| ProductRelations::Videos(data.into_public()))
                    .boxed(),
            )
        }

        if relations.reviews {
            futures.push(
                self.product_review_repository
                    .get_all_for_multiple_products(&product_ids)
                    .map_ok(|data| ProductRelations::Reviews(data.into_public()))
                    .boxed(),
            )
        }

        try_join_all(futures).await
    }
}
