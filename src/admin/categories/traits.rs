use crate::admin::categories::dto::AdminPublicCategory;
use crate::admin::categories::model::AdminCategoryModel;
use crate::utils::pagination::{DataCollection, PaginatedDataCollection};

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<AdminPublicCategory> for AdminCategoryModel {
    fn into_public(self) -> AdminPublicCategory {
        AdminPublicCategory::from(self)
    }
}

impl IntoPublic<DataCollection<AdminPublicCategory>> for DataCollection<AdminCategoryModel> {
    fn into_public(self) -> DataCollection<AdminPublicCategory> {
        DataCollection::new(
            self.data
                .into_iter()
                .map(AdminPublicCategory::from)
                .collect(),
        )
    }
}

impl IntoPublic<PaginatedDataCollection<AdminPublicCategory>>
    for PaginatedDataCollection<AdminCategoryModel>
{
    fn into_public(self) -> PaginatedDataCollection<AdminPublicCategory> {
        PaginatedDataCollection::new(
            self.data
                .into_iter()
                .map(AdminPublicCategory::from)
                .collect(),
            self.pagination,
        )
    }
}
