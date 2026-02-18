use crate::admin::products::images::dto::AdminPublicProductImage;
use crate::admin::products::images::model::AdminProductImageModel;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<AdminPublicProductImage> for AdminProductImageModel {
    fn into_public(self) -> AdminPublicProductImage {
        AdminPublicProductImage::from(self)
    }
}

impl IntoPublic<Vec<AdminPublicProductImage>> for Vec<AdminProductImageModel> {
    fn into_public(self) -> Vec<AdminPublicProductImage> {
        self.into_iter()
            .map(AdminPublicProductImage::from)
            .collect()
    }
}
