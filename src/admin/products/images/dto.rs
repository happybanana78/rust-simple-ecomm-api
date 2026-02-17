use crate::admin::products::images::model::AdminProductImageModel;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminPublicProductImage {
    pub id: i64,
    pub product_id: i64,
    pub url: String,
    pub alt: String,
    pub is_main: bool,
    pub sort: i32,
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
    pub url: String,
    pub alt: String,
    pub sort: i32,
    pub is_main: bool,
}

impl CreateProductImageCommand {
    pub fn new_from_dto(dto: CreateProductImageDTO, url: String) -> Self {
        Self {
            product_id: dto.product_id.into_inner(),
            url,
            alt: dto.alt.into_inner(),
            sort: dto.sort.into_inner(),
            is_main: dto.is_main.into_inner(),
        }
    }
}
