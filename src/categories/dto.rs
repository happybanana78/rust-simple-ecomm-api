use crate::categories::model::CategoryModel;
use crate::traits::HasId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicCategory {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
}

impl HasId for PublicCategory {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<CategoryModel> for PublicCategory {
    fn from(category: CategoryModel) -> Self {
        Self {
            id: category.id,
            name: category.name,
            slug: category.slug,
            is_active: category.is_active,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct IndexCategoryDTO {
    #[validate(required, range(min = 1))]
    pub page: Option<i64>,

    #[validate(required, range(min = 1))]
    pub limit: Option<i64>,

    #[validate(length(min = 1))]
    pub search: Option<String>,
}
