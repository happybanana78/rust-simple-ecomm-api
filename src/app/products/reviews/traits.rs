use crate::app::products::reviews::dto::PublicProductReview;
use crate::app::products::reviews::model::ProductReviewModel;

pub trait IntoPublic<T> {
    fn into_public(self) -> T;
}

impl IntoPublic<PublicProductReview> for ProductReviewModel {
    fn into_public(self) -> PublicProductReview {
        PublicProductReview::from(self)
    }
}

impl IntoPublic<Vec<PublicProductReview>> for Vec<ProductReviewModel> {
    fn into_public(self) -> Vec<PublicProductReview> {
        self.into_iter().map(PublicProductReview::from).collect()
    }
}
