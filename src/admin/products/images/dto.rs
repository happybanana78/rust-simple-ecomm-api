use crate::admin::products::images::model::AdminProductImageModel;
use crate::traits::HasId;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminPublicProductImage {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: i32,
}

impl HasId for AdminPublicProductImage {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<AdminProductImageModel> for AdminPublicProductImage {
    fn from(image: AdminProductImageModel) -> Self {
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
pub struct CreateProductImageDTO {
    pub product_id: Text<i64>,
    pub file: TempFile,
    pub alt: Text<String>,
    pub sort: Text<i32>,
    pub is_main: Text<bool>,
}

pub struct CreateProductImageCommand {
    pub product_id: i64,
    pub url: Option<String>,
    pub alt: String,
    pub sort: i32,
    pub is_main: bool,
}

impl CreateProductImageCommand {
    pub fn new_from_dto(dto: &CreateProductImageDTO) -> Self {
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
