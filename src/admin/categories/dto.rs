use crate::admin::categories::filters::CategoryFilters;
use crate::admin::categories::model::AdminCategoryModel;
use crate::errors::error::AppError;
use crate::utils::traits::HasId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminPublicCategory {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
}

impl HasId for AdminPublicCategory {
    fn get_id(&self) -> i64 {
        self.id
    }
}

impl From<AdminCategoryModel> for AdminPublicCategory {
    fn from(category: AdminCategoryModel) -> Self {
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

    pub is_active: Option<bool>,
}

impl TryFrom<IndexCategoryDTO> for CategoryFilters {
    type Error = AppError;

    fn try_from(dto: IndexCategoryDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            is_active: dto.is_active,
        })
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateCategoryDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, length(min = 1))]
    pub slug: Option<String>,

    pub is_active: Option<bool>,
}

pub struct CreateCategoryCommand {
    pub name: String,
    pub slug: String,
    pub is_active: bool,
}

impl TryFrom<CreateCategoryDTO> for CreateCategoryCommand {
    type Error = AppError;

    fn try_from(dto: CreateCategoryDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name.unwrap(),
            slug: dto.slug.unwrap(),
            is_active: dto.is_active.unwrap_or(true),
        })
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateCategoryDTO {
    #[validate(required, length(min = 3))]
    pub name: Option<String>,

    #[validate(required, length(min = 1))]
    pub slug: Option<String>,

    pub is_active: Option<bool>,
}

pub struct UpdateCategoryCommand {
    pub name: String,
    pub slug: String,
    pub is_active: bool,
}

impl TryFrom<UpdateCategoryDTO> for UpdateCategoryCommand {
    type Error = AppError;

    fn try_from(dto: UpdateCategoryDTO) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name.unwrap(),
            slug: dto.slug.unwrap(),
            is_active: dto.is_active.unwrap_or(true),
        })
    }
}
