use crate::admin::products::service::AdminProductService;
use crate::admin::products::videos::dto::CreateProductVideoCommand;
use crate::admin::products::videos::model::AdminProductVideoModel;
use crate::admin::products::videos::repository::AdminProductVideoRepository;
use crate::errors::error::AppError;
use crate::storage::LocalStorage;
use crate::traits::{IsRepository, UseStorage};
use actix_files::NamedFile;
use bytes::Bytes;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AdminProductVideoService {
    repository: AdminProductVideoRepository,
    product_service: AdminProductService,
}

impl AdminProductVideoService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminProductVideoRepository::new(pool.clone()),
            product_service: AdminProductService::new(pool),
        }
    }

    pub async fn get_one(&self, id: i64) -> Result<AdminProductVideoModel, AppError> {
        let video = self.repository.show(id).await?;

        match video {
            Some(video) => Ok(video),
            None => Err(AppError::NotFound("Video not found".to_string())),
        }
    }

    pub async fn upload(
        &self,
        mut cmd: CreateProductVideoCommand,
        storage: &LocalStorage,
        file_bytes: Vec<u8>,
        extension: &str,
    ) -> Result<i64, AppError> {
        self.product_service.get_one(cmd.product_id).await?;

        let file_name = format!("product-video-{}-{}", cmd.product_id, Uuid::new_v4());

        let url = storage
            .upload(file_name.as_str(), extension, Bytes::from(file_bytes))
            .await?;

        cmd.set_url(url);
        cmd.handle_main(&self.repository).await?;
        cmd.handle_sort(&self.repository).await?;

        let video_model = self
            .repository
            .create(self.repository.get_pool(), &cmd)
            .await?;

        Ok(video_model.id)
    }

    pub async fn stream(&self, id: i64) -> Result<NamedFile, AppError> {
        let video = self.get_one(id).await?;
        NamedFile::open(format!("./{}", video.url)).map_err(|e| AppError::Internal(e.to_string()))
    }

    pub async fn delete(&self, id: i64, storage: &LocalStorage) -> Result<u64, AppError> {
        let video = self.get_one(id).await?;
        storage.delete(video.url.as_str()).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }
}
