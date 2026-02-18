use crate::products::images::dto::PublicProductImage;
use crate::products::images::model::ProductImageModel;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<PublicProductImage> for ProductImageModel {
    fn into_public(self) -> PublicProductImage {
        PublicProductImage::from(self)
    }
}

impl IntoPublic<Vec<PublicProductImage>> for Vec<ProductImageModel> {
    fn into_public(self) -> Vec<PublicProductImage> {
        self.into_iter().map(PublicProductImage::from).collect()
    }
}
