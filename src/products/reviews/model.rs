use crate::admin::reviews::dto::ReviewApprovalStatus;
use crate::traits::HasId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductReviewModel {
    pub id: i64,
    pub user_id: Option<i64>,
    pub product_id: i64,
    pub title: String,
    pub content: String,
    pub rating: i16,
    pub approval_status: ReviewApprovalStatus,
    pub created_at: DateTime<Utc>,
}

impl HasId for ProductReviewModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}
