use crate::pagination::PaginatedDataCollection;
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

impl IntoPublic<PaginatedDataCollection<PublicProduct>> for PaginatedDataCollection<ProductModel> {
    fn into_public(self) -> PaginatedDataCollection<PublicProduct> {
        PaginatedDataCollection::new(
            self.data.into_iter().map(PublicProduct::from).collect(),
            self.pagination,
        )
    }
}
