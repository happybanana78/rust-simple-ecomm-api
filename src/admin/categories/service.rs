use crate::admin::categories::dto::{
    AdminPublicCategory, CreateCategoryCommand, UpdateCategoryCommand,
};
use crate::admin::categories::filters::CategoryFilters;
use crate::admin::categories::model::AdminCategoryModel;
use crate::admin::categories::repository::AdminCategoryRepository;
use crate::admin::categories::traits::IntoPublic;
use crate::errors::error::AppError;
use crate::pagination::{DataCollection, Paginate, PaginatedDataCollection};
use crate::validation_utils::validate_slug;
use sqlx::PgPool;

pub struct AdminCategoryService {
    repository: AdminCategoryRepository,
}

impl AdminCategoryService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminCategoryRepository::new(pool),
        }
    }

    pub async fn get_all(&self) -> Result<DataCollection<AdminCategoryModel>, AppError> {
        let data = self.repository.index().await?;
        Ok(DataCollection::new(data))
    }

    pub async fn get_all_paginated(
        &self,
        pagination: &Paginate,
        filters: &CategoryFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<AdminCategoryModel>, AppError> {
        let data = self
            .repository
            .index_paginated(pagination, search, filters)
            .await?;
        Ok(PaginatedDataCollection::new(data, pagination.clone()))
    }

    pub async fn get_all_public(&self) -> Result<DataCollection<AdminPublicCategory>, AppError> {
        let data = self.get_all().await?;
        Ok(data.into_public())
    }

    pub async fn get_all_paginated_public(
        &self,
        pagination: &Paginate,
        filters: &CategoryFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<AdminPublicCategory>, AppError> {
        let data = self.get_all_paginated(pagination, filters, search).await?;
        Ok(data.into_public())
    }

    pub async fn get_one(&self, id: i64) -> Result<AdminCategoryModel, AppError> {
        let category = self.repository.show(id).await?;

        match category {
            Some(category) => Ok(category),
            None => Err(AppError::NotFound("Category not found".to_string())),
        }
    }

    pub async fn get_one_public(&self, id: i64) -> Result<AdminPublicCategory, AppError> {
        let category = self.get_one(id).await?;

        Ok(category.into_public())
    }

    pub async fn create(&self, cmd: CreateCategoryCommand) -> Result<AdminCategoryModel, AppError> {
        validate_slug(&cmd.slug)?;

        let category_already_exists = self.check_exist_with_same_name(&cmd.name).await?;
        let slug_already_exists = self.check_exist_with_same_slug(&cmd.slug).await?;

        if category_already_exists {
            return Err(AppError::Conflict(
                "Category with the same name already exists".to_string(),
            ));
        }

        if slug_already_exists {
            return Err(AppError::Conflict(
                "Category with the same slug already exists".to_string(),
            ));
        }

        self.repository.create(cmd).await
    }

    pub async fn update(&self, cmd: UpdateCategoryCommand, id: i64) -> Result<u64, AppError> {
        self.get_one(id).await?;
        validate_slug(&cmd.slug)?;

        let category_already_exists = self.check_exist_with_same_name(&cmd.name).await?;
        let slug_already_exists = self.check_exist_with_same_slug(&cmd.slug).await?;

        if category_already_exists {
            return Err(AppError::Conflict(
                "Category with the same name already exists".to_string(),
            ));
        }

        if slug_already_exists {
            return Err(AppError::Conflict(
                "Category with the same slug already exists".to_string(),
            ));
        }

        self.repository.update(cmd, id).await
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        self.get_one(id).await?;
        self.repository.delete(id).await
    }

    pub async fn check_exist_with_same_name(&self, name: &str) -> Result<bool, AppError> {
        self.repository.check_existence_by_name(name).await
    }

    pub async fn check_exist_with_same_slug(&self, slug: &str) -> Result<bool, AppError> {
        self.repository.check_existence_by_slug(slug).await
    }
}
