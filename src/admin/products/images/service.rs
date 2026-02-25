use crate::admin::products::images::dto::{
    CreateProductImageCommand, UpdateProductImageSortCommand,
};
use crate::admin::products::images::model::AdminProductImageModel;
use crate::admin::products::images::repository::AdminProductImageRepository;
use crate::admin::products::service::AdminProductService;
use crate::errors::error::AppError;
use crate::storage::LocalStorage;
use crate::traits::{IsRepository, UseStorage};
use bigdecimal::BigDecimal;
use bytes::Bytes;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AdminProductImageService {
    repository: AdminProductImageRepository,
    product_service: AdminProductService,
}

impl AdminProductImageService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminProductImageRepository::new(pool.clone()),
            product_service: AdminProductService::new(pool),
        }
    }

    pub async fn get_one(&self, id: i64) -> Result<AdminProductImageModel, AppError> {
        let image = self.repository.show(id).await?;

        match image {
            Some(image) => Ok(image),
            None => Err(AppError::NotFound("Image not found".to_string())),
        }
    }

    pub async fn upload(
        &self,
        mut cmd: CreateProductImageCommand,
        storage: &LocalStorage,
        file_bytes: Vec<u8>,
        extension: &str,
    ) -> Result<i64, AppError> {
        self.product_service.get_one(cmd.product_id).await?;

        let file_name = format!("product-image-{}-{}", cmd.product_id, Uuid::new_v4());

        let url = storage
            .upload(file_name.as_str(), extension, Bytes::from(file_bytes))
            .await?;

        cmd.set_url(url);
        cmd.handle_main(&self.repository).await?;
        cmd.handle_sort(&self.repository).await?;

        let image_model = self
            .repository
            .create(self.repository.get_pool(), &cmd)
            .await?;

        Ok(image_model.id)
    }

    pub async fn update_sort(
        &self,
        id: i64,
        cmd: UpdateProductImageSortCommand,
    ) -> Result<BigDecimal, AppError> {
        let image = self.get_one(id).await?;

        let mut sort_images = self
            .repository
            .get_images_only_sort(image.product_id)
            .await?;
        sort_images.retain(|i| i.id != id);

        if sort_images.is_empty() {
            return Ok(BigDecimal::from(0));
        }

        let divider = BigDecimal::from(2);
        let amount_to_add = BigDecimal::from(1000);

        let new_sort = if cmd.target_index == 0 {
            let first = sort_images.first().unwrap();
            first.sort.clone() / divider
        } else if cmd.target_index >= sort_images.len() {
            let last = sort_images.last().unwrap();
            last.sort.clone() + amount_to_add
        } else {
            let prev = &sort_images[cmd.target_index - 1];
            let next = &sort_images[cmd.target_index];

            (prev.sort.clone() + next.sort.clone()) / divider
        };

        self.repository.update_sort(id, new_sort.clone()).await?;

        Ok(new_sort)
    }

    pub async fn delete(&self, id: i64, storage: &LocalStorage) -> Result<u64, AppError> {
        let image = self.get_one(id).await?;
        storage.delete(image.url.as_str()).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }
}
