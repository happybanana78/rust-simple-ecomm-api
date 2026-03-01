use crate::products::images::dto::PublicProductImage;
use crate::products::reviews::dto::PublicProductReview;
use crate::products::videos::dto::PublicProductVideo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductLoadRelations {
    pub images: bool,
    pub videos: bool,
    pub reviews: bool,
}

#[derive(Debug)]
pub enum ProductRelations {
    Images(Vec<PublicProductImage>),
    Videos(Vec<PublicProductVideo>),
    Reviews(Vec<PublicProductReview>),
}
