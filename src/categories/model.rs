use crate::traits::HasId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct CategoryModel {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
}

impl HasId for CategoryModel {
    fn get_id(&self) -> i64 {
        self.id
    }
}
