use crate::admin::products::images::dto::CreateProductImageCommand;
use crate::admin::products::images::model::AdminProductImageModel;
use crate::admin::products::images::repository::AdminProductImageRepository;
use crate::admin::products::service::AdminProductService;
use crate::errors::error::AppError;
use crate::storage::LocalStorage;
use crate::traits::{IsRepository, UseStorage};
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
    ) -> Result<(), AppError> {
        self.product_service.get_one(cmd.product_id).await?;

        let file_name = format!("product-image-{}-{}", cmd.product_id, Uuid::new_v4());

        let url = storage
            .upload(file_name.as_str(), extension, Bytes::from(file_bytes))
            .await?;

        cmd.set_url(url);
        cmd.handle_main(&self.repository).await?;
        cmd.handle_sort(&self.repository).await?;

        self.repository
            .create(self.repository.get_pool(), &cmd)
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64, storage: &LocalStorage) -> Result<u64, AppError> {
        let image = self.get_one(id).await?;
        storage.delete(image.url.as_str()).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }
}
