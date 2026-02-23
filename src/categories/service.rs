use crate::categories::dto::PublicCategory;
use crate::categories::model::CategoryModel;
use crate::categories::repository::CategoryRepository;
use crate::categories::traits::IntoPublic;
use crate::errors::error::AppError;
use crate::pagination::{Paginate, PaginatedDataCollection};
use sqlx::PgPool;

pub struct CategoryService {
    repository: CategoryRepository,
}

impl CategoryService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: CategoryRepository::new(pool),
        }
    }

    pub async fn get_all_paginated(
        &self,
        pagination: &Paginate,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<CategoryModel>, AppError> {
        let data = self.repository.index_paginated(pagination, search).await?;
        Ok(PaginatedDataCollection::new(data, pagination.clone()))
    }

    pub async fn get_all_paginated_public(
        &self,
        pagination: &Paginate,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<PublicCategory>, AppError> {
        let data = self.get_all_paginated(pagination, search).await?;
        Ok(data.into_public())
    }

    pub async fn get_one(&self, slug: &str) -> Result<CategoryModel, AppError> {
        let category = self.repository.show(slug).await?;

        match category {
            Some(category) => Ok(category),
            None => Err(AppError::NotFound("Category not found".to_string())),
        }
    }

    pub async fn get_one_public(&self, slug: &str) -> Result<PublicCategory, AppError> {
        let category = self.get_one(slug).await?;

        Ok(category.into_public())
    }
}
