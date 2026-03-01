use crate::app::categories::dto::PublicCategory;
use crate::app::categories::model::CategoryModel;
use crate::pagination::PaginatedDataCollection;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<PublicCategory> for CategoryModel {
    fn into_public(self) -> PublicCategory {
        PublicCategory::from(self)
    }
}

impl IntoPublic<PaginatedDataCollection<PublicCategory>>
    for PaginatedDataCollection<CategoryModel>
{
    fn into_public(self) -> PaginatedDataCollection<PublicCategory> {
        PaginatedDataCollection::new(
            self.data.into_iter().map(PublicCategory::from).collect(),
            self.pagination,
        )
    }
}
