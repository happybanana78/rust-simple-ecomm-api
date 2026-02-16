use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryFilters {
    pub is_active: Option<bool>,
}
