use crate::admin::products::videos::model::AdminProductVideoModel;
use crate::admin::products::videos::repository::AdminProductVideoRepository;
use crate::errors::error::AppError;
use crate::traits::HasId;
use crate::validation_utils::validate_target_index;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminPublicProductVideo {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: BigDecimal,
}

impl HasId for AdminPublicProductVideo {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<AdminProductVideoModel> for AdminPublicProductVideo {
    fn from(image: AdminProductVideoModel) -> Self {
        Self {
            id: image.id,
            product_id: image.product_id,
            url: image.url,
            alt: image.alt,
            is_main: image.is_main,
            sort: image.sort,
        }
    }
}

#[derive(MultipartForm)]
pub struct CreateProductVideoDTO {
    pub product_id: Text<i64>,
    pub file: TempFile,
    pub alt: Text<String>,
    pub is_main: Text<bool>,
}

pub struct CreateProductVideoCommand {
    pub product_id: i64,
    pub url: Option<String>,
    pub alt: String,
    pub sort: BigDecimal,
    pub is_main: bool,
}

impl CreateProductVideoCommand {
    pub fn new_from_dto(dto: &CreateProductVideoDTO) -> Self {
        Self {
            product_id: *dto.product_id,
            url: None,
            alt: dto.alt.clone(),
            sort: BigDecimal::from(1000),
            is_main: *dto.is_main,
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    /**
     * Asynchronously handles the logic for determining and modifying the "main video" status
     * of a product based on the given parameters and repository operations.
     */
    pub async fn handle_main(
        &mut self,
        repository: &AdminProductVideoRepository,
    ) -> Result<(), AppError> {
        let total_count = repository.get_total_count(self.product_id).await?;

        if total_count == 0 {
            self.is_main = true;
        } else if self.is_main && total_count > 1 {
            repository.reset_is_main(self.product_id).await?;
            self.is_main = true;
        }

        Ok(())
    }

    /**
     * Handles the sorting logic for a product video.
     */
    pub async fn handle_sort(
        &mut self,
        repository: &AdminProductVideoRepository,
    ) -> Result<(), AppError> {
        let last_sort = repository.get_last_sort(self.product_id).await?;

        let default_sort = BigDecimal::from(1000);

        match last_sort {
            Some(sort) => self.sort = sort + default_sort,
            None => self.sort = default_sort,
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductVideoSortDTO {
    pub target_index: Option<i32>,
}

pub struct UpdateProductVideoSortCommand {
    pub target_index: usize,
}

impl UpdateProductVideoSortCommand {
    pub fn new_from_dto(dto: &UpdateProductVideoSortDTO) -> Result<Self, AppError> {
        let target_index = validate_target_index(dto.target_index)?;

        Ok(Self { target_index })
    }
}
