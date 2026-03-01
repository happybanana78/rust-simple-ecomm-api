use crate::admin::reviews::dto::ReviewApprovalStatus;
use crate::app::products::reviews::model::ProductReviewModel;
use crate::traits::HasId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicProductReview {
    pub id: i64,
    pub user_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub content: String,
    pub rating: i16,
    pub approval_status: ReviewApprovalStatus,
}

impl HasId for PublicProductReview {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<ProductReviewModel> for PublicProductReview {
    fn from(review: ProductReviewModel) -> Self {
        Self {
            id: review.id,
            user_id: review.user_id,
            product_id: review.product_id,
            title: review.title,
            content: review.content,
            rating: review.rating,
            approval_status: review.approval_status,
        }
    }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductReviewDto {
    #[validate(required, range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(required, length(min = 1))]
    pub title: Option<String>,

    #[validate(required, length(min = 1))]
    pub content: Option<String>,

    #[validate(required, range(min = 1))]
    pub rating: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProductReviewCommand {
    pub user_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub content: String,
    pub rating: i16,
    pub status: ReviewApprovalStatus,
}

impl CreateProductReviewCommand {
    pub fn from_dto(dto: CreateProductReviewDto, user_id: Option<i64>) -> Self {
        Self {
            user_id,
            product_id: dto.product_id.unwrap(),
            title: dto.title.unwrap(),
            content: dto.content.unwrap(),
            rating: dto.rating.unwrap(),
            status: ReviewApprovalStatus::Pending,
        }
    }
}
