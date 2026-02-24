use crate::admin::products::service::AdminProductService;
use crate::admin::products::videos::dto::{CreateProductVideoCommand, CreateProductVideoDTO};
use crate::admin::products::videos::model::AdminProductVideoModel;
use crate::admin::products::videos::repository::AdminProductVideoRepository;
use crate::errors::error::AppError;
use crate::storage::LocalStorage;
use crate::traits::{IsRepository, UseStorage};
use actix_files::NamedFile;
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
        dto: CreateProductVideoDTO,
        storage: &LocalStorage,
    ) -> Result<(), AppError> {
        self.product_service.get_one(*dto.product_id).await?;

        let mut command = CreateProductVideoCommand::new_from_dto(&dto, &self.repository).await?;

        if self.repository.get_total_count(*dto.product_id).await? == 0 {
            command.set_is_main(true);
        }

        let file_name = format!("product-video-{}-{}", *dto.product_id, Uuid::new_v4());

        let url = storage
            .upload_from_temp(file_name.as_str(), dto.file)
            .await?;

        command.set_url(url);

        self.repository
            .create(self.repository.get_pool(), &command)
            .await?;

        Ok(())
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
