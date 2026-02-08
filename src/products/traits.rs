use crate::products::dto::PublicProduct;
use crate::products::model::ProductModel;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<PublicProduct> for ProductModel {
    fn into_public(self) -> PublicProduct {
        PublicProduct::from(self)
    }
}

impl IntoPublic<Vec<PublicProduct>> for Vec<ProductModel> {
    fn into_public(self) -> Vec<PublicProduct> {
        self.into_iter().map(PublicProduct::from).collect()
    }
}
