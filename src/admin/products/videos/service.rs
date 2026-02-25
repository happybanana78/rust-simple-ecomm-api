use crate::admin::products::service::AdminProductService;
use crate::admin::products::videos::dto::{
    CreateProductVideoCommand, UpdateProductVideoSortCommand,
};
use crate::admin::products::videos::model::AdminProductVideoModel;
use crate::admin::products::videos::repository::AdminProductVideoRepository;
use crate::errors::error::AppError;
use crate::storage::LocalStorage;
use crate::traits::{IsRepository, UseStorage};
use actix_files::NamedFile;
use bigdecimal::BigDecimal;
use bytes::Bytes;
use sqlx::PgPool;
use std::path::Path;
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

    pub async fn update_sort(
        &self,
        id: i64,
        cmd: UpdateProductVideoSortCommand,
    ) -> Result<BigDecimal, AppError> {
        let video = self.get_one(id).await?;

        let mut sort_videos = self
            .repository
            .get_videos_only_sort(video.product_id)
            .await?;
        sort_videos.retain(|i| i.id != id);

        if sort_videos.is_empty() {
            return Ok(BigDecimal::from(0));
        }

        let divider = BigDecimal::from(2);
        let amount_to_add = BigDecimal::from(1000);

        let new_sort = if cmd.target_index == 0 {
            let first = sort_videos.first().unwrap();
            first.sort.clone() / divider
        } else if cmd.target_index >= sort_videos.len() {
            let last = sort_videos.last().unwrap();
            last.sort.clone() + amount_to_add
        } else {
            let prev = &sort_videos[cmd.target_index - 1];
            let next = &sort_videos[cmd.target_index];

            (prev.sort.clone() + next.sort.clone()) / divider
        };

        self.repository.update_sort(id, new_sort.clone()).await?;

        Ok(new_sort)
    }

    pub async fn stream(&self, id: i64) -> Result<NamedFile, AppError> {
        let video = self.get_one(id).await?;

        let full_path = Path::new("./").join(&video.url);

        NamedFile::open(full_path).map_err(|e| AppError::Internal(e.to_string()))
    }

    pub async fn delete(&self, id: i64, storage: &LocalStorage) -> Result<u64, AppError> {
        let video = self.get_one(id).await?;
        storage.delete(video.url.as_str()).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }
}
