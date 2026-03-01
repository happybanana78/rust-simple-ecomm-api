use crate::app::products::filters::ProductFilters;
use crate::app::products::images::dto::PublicProductImage;
use crate::app::products::model::ProductModel;
use crate::app::products::relations::ProductLoadRelations;
use crate::app::products::reviews::dto::PublicProductReview;
use crate::app::products::traits::IntoPublic;
use crate::app::products::videos::dto::PublicProductVideo;
use crate::errors::error::AppError;
use crate::traits::{HasId, HasQuantity};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicProductBuilder {
    pub product: PublicProduct,
}

impl PublicProductBuilder {
    pub fn new(product: &ProductModel) -> Self {
        Self {
            product: product.clone().into_public(),
        }
    }

    pub fn with_images(mut self, images: Vec<PublicProductImage>) -> Self {
        self.product.images = Some(images);
        self
    }

    pub fn with_videos(mut self, videos: Vec<PublicProductVideo>) -> Self {
        self.product.videos = Some(videos);
        self
    }

    pub fn with_reviews(mut self, reviews: Vec<PublicProductReview>) -> Self {
        self.product.reviews = Some(reviews);
        self
    }

    pub fn build(self) -> PublicProduct {
        self.product
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicProduct {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub price: f64,
    pub quantity: i32,
    pub configurable: bool,
    pub is_active: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<PublicProductImage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<PublicProductVideo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Vec<PublicProductReview>>,
}

impl HasId for PublicProduct {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<ProductModel> for PublicProduct {
    fn from(product: ProductModel) -> Self {
        Self {
            id: product.id,
            name: product.name,
            slug: product.slug,
            price: product.price,
            quantity: product.quantity,
            configurable: product.configurable,
            is_active: product.is_active,
            images: None,
            videos: None,
            reviews: None,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct IndexProductDTO {
    #[validate(required, range(min = 1))]
    pub page: Option<i64>,

    #[validate(required, range(min = 1))]
    pub limit: Option<i64>,

    #[validate(length(min = 1))]
    pub search: Option<String>,

    // filters
    #[validate(range(min = 1))]
    pub category: Option<i64>,

    #[validate(range(min = 0.0))]
    pub price_min: Option<f64>,

    #[validate(range(min = 0.0))]
    pub price_max: Option<f64>,

    // relations
    pub images: Option<bool>,
    pub videos: Option<bool>,
    pub reviews: Option<bool>,
}

impl TryFrom<IndexProductDTO> for ProductFilters {
    type Error = AppError;

    fn try_from(dto: IndexProductDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            category: dto.category,
            price_min: dto.price_min,
            price_max: dto.price_max,
        })
    }
}

impl From<IndexProductDTO> for ProductLoadRelations {
    fn from(dto: IndexProductDTO) -> Self {
        Self {
            images: dto.images.is_some(),
            videos: dto.videos.is_some(),
            reviews: dto.reviews.is_some(),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct ShowProductDTO {
    pub images: Option<bool>,
    pub videos: Option<bool>,
    pub reviews: Option<bool>,
}

impl From<ShowProductDTO> for ProductLoadRelations {
    fn from(dto: ShowProductDTO) -> Self {
        Self {
            images: dto.images.is_some(),
            videos: dto.videos.is_some(),
            reviews: dto.reviews.is_some(),
        }
    }
}

pub struct UpdateStockDto {
    pub product_id: i64,
    pub quantity: i32,
}

impl UpdateStockDto {
    pub fn new(product_id: i64, quantity: i32) -> Self {
        Self {
            product_id,
            quantity,
        }
    }
}

impl HasQuantity for UpdateStockDto {
    fn get_quantity(&self) -> i32 {
        self.quantity
    }
}
