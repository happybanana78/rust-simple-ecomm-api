use crate::admin::products::images::dto::{CreateProductImageCommand, CreateProductImageDTO};
use crate::admin::products::images::model::AdminProductImageModel;
use crate::admin::products::images::repository::AdminProductImageRepository;
use crate::admin::products::service::AdminProductService;
use crate::errors::error::AppError;
use crate::pagination::DataCollection;
use crate::traits::IsRepository;
use sqlx::PgPool;

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

    pub async fn get_all(&self) -> Result<DataCollection<AdminProductImageModel>, AppError> {
        let data = self.repository.index().await?;
        Ok(DataCollection::new(data))
    }

    pub async fn get_one(&self, id: i64) -> Result<AdminProductImageModel, AppError> {
        let image = self.repository.show(id).await?;

        match image {
            Some(image) => Ok(image),
            None => Err(AppError::NotFound("Image not found".to_string())),
        }
    }

    pub async fn upload(&self, dto: CreateProductImageDTO) -> Result<(), AppError> {
        self.product_service.get_one(*dto.product_id).await?;

        // upload
        let url = "https://example.com/image.jpg".to_string();

        // create
        let command = CreateProductImageCommand::new_from_dto(dto, url);
        self.repository
            .create(self.repository.get_pool(), &command)
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        self.get_one(id).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }
}
