use crate::admin::reviews::dto::AdminPublicReview;
use crate::admin::reviews::model::AdminReviewModel;
use crate::utils::pagination::PaginatedDataCollection;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<AdminPublicReview> for AdminReviewModel {
    fn into_public(self) -> AdminPublicReview {
        AdminPublicReview::from(self)
    }
}

impl IntoPublic<PaginatedDataCollection<AdminPublicReview>>
    for PaginatedDataCollection<AdminReviewModel>
{
    fn into_public(self) -> PaginatedDataCollection<AdminPublicReview> {
        PaginatedDataCollection::new(
            self.data.into_iter().map(AdminPublicReview::from).collect(),
            self.pagination,
        )
    }
}
