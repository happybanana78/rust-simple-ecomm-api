use crate::admin::products::dto::AdminPublicProduct;
use crate::admin::products::model::AdminProductModel;
use crate::pagination::{DataCollection, PaginatedDataCollection};

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<AdminPublicProduct> for AdminProductModel {
    fn into_public(self) -> AdminPublicProduct {
        AdminPublicProduct::from(self)
    }
}

impl IntoPublic<DataCollection<AdminPublicProduct>> for DataCollection<AdminProductModel> {
    fn into_public(self) -> DataCollection<AdminPublicProduct> {
        DataCollection::new(
            self.data
                .into_iter()
                .map(AdminPublicProduct::from)
                .collect(),
        )
    }
}

impl IntoPublic<PaginatedDataCollection<AdminPublicProduct>>
    for PaginatedDataCollection<AdminProductModel>
{
    fn into_public(self) -> PaginatedDataCollection<AdminPublicProduct> {
        PaginatedDataCollection::new(
            self.data
                .into_iter()
                .map(AdminPublicProduct::from)
                .collect(),
            self.pagination,
        )
    }
}
