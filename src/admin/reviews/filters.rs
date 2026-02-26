use crate::admin::reviews::dto::ReviewApprovalStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminReviewFilters {
    pub user_id: Option<i64>,
    pub product_id: Option<i64>,
    pub rating: Option<i16>,
    pub approval_status: Option<ReviewApprovalStatus>,
}
