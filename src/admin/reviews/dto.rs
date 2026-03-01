use crate::admin::reviews::filters::AdminReviewFilters;
use crate::admin::reviews::model::AdminReviewModel;
use crate::errors::error::AppError;
use crate::traits::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "review_status", rename_all = "lowercase")]
pub enum ReviewApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl FromStr for ReviewApprovalStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ReviewApprovalStatus::Pending),
            "approved" => Ok(ReviewApprovalStatus::Approved),
            "rejected" => Ok(ReviewApprovalStatus::Rejected),
            _ => {
                let mut error = HashMap::new();
                error.insert("status".to_string(), vec!["invalid status".to_string()]);
                Err(AppError::ValidationSingle(error))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminPublicReview {
    pub id: i64,
    pub user_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub content: String,
    pub rating: i16,
    pub approval_status: ReviewApprovalStatus,
}

impl HasId for AdminPublicReview {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<AdminReviewModel> for AdminPublicReview {
    fn from(review: AdminReviewModel) -> Self {
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

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct IndexReviewDTO {
    #[validate(required, range(min = 1))]
    pub page: Option<i64>,

    #[validate(required, range(min = 1))]
    pub limit: Option<i64>,

    #[validate(length(min = 1))]
    pub search: Option<String>,

    #[validate(range(min = 1))]
    pub user_id: Option<i64>,

    #[validate(range(min = 1))]
    pub product_id: Option<i64>,

    #[validate(range(min = 0))]
    pub rating: Option<i16>,

    #[validate(length(min = 1))]
    pub status: Option<String>,
}

impl TryFrom<IndexReviewDTO> for AdminReviewFilters {
    type Error = AppError;

    fn try_from(dto: IndexReviewDTO) -> Result<Self, Self::Error> {
        let status = match dto.status {
            Some(status) => Some(ReviewApprovalStatus::from_str(&status)?),
            None => None,
        };

        Ok(Self {
            user_id: dto.user_id,
            product_id: dto.product_id,
            rating: dto.rating,
            approval_status: status,
        })
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct UpdateReviewStatusDTO {
    #[validate(required, length(min = 1))]
    pub status: Option<String>,
}

pub struct UpdateReviewStatusCommand {
    pub status: ReviewApprovalStatus,
}

impl TryFrom<UpdateReviewStatusDTO> for UpdateReviewStatusCommand {
    type Error = AppError;

    fn try_from(dto: UpdateReviewStatusDTO) -> Result<Self, Self::Error> {
        let status = ReviewApprovalStatus::from_str(dto.status.unwrap().as_str())?;
        Ok(Self { status })
    }
}
