use crate::admin::products::videos::model::AdminProductVideoModel;
use crate::traits::HasId;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminPublicProductVideo {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: i32,
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
    pub sort: Text<i32>,
    pub is_main: Text<bool>,
}

pub struct CreateProductVideoCommand {
    pub product_id: i64,
    pub url: Option<String>,
    pub alt: String,
    pub sort: i32,
    pub is_main: bool,
}

impl CreateProductVideoCommand {
    pub fn new_from_dto(dto: &CreateProductVideoDTO) -> Self {
        Self {
            product_id: *dto.product_id,
            url: None,
            alt: dto.alt.clone(),
            sort: *dto.sort,
            is_main: *dto.is_main,
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn set_is_main(&mut self, value: bool) {
        self.is_main = value;
    }
}
