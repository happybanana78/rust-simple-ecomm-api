use crate::admin::reviews::dto::UpdateReviewStatusCommand;
use crate::admin::reviews::dto::{AdminPublicReview, ReviewApprovalStatus};
use crate::admin::reviews::filters::AdminReviewFilters;
use crate::admin::reviews::model::AdminReviewModel;
use crate::admin::reviews::repository::AdminReviewRepository;
use crate::admin::reviews::traits::IntoPublic;
use crate::errors::error::AppError;
use crate::utils::pagination::{Paginate, PaginatedDataCollection};
use crate::utils::traits::IsRepository;
use sqlx::PgPool;

pub struct AdminReviewService {
    repository: AdminReviewRepository,
}

impl AdminReviewService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AdminReviewRepository::new(pool),
        }
    }

    pub async fn get_all_paginated(
        &self,
        pagination: &Paginate,
        filters: &AdminReviewFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<AdminReviewModel>, AppError> {
        let data = self
            .repository
            .index_paginated(pagination, search, filters)
            .await?;
        Ok(PaginatedDataCollection::new(data, pagination.clone()))
    }

    /**
     * Get all paginated reviews (public version).
     */
    pub async fn get_all_paginated_public(
        &self,
        pagination: &Paginate,
        filters: &AdminReviewFilters,
        search: &Option<String>,
    ) -> Result<PaginatedDataCollection<AdminPublicReview>, AppError> {
        let reviews = self.get_all_paginated(pagination, filters, search).await?;
        Ok(reviews.into_public())
    }

    pub async fn get_one(&self, id: i64) -> Result<AdminReviewModel, AppError> {
        let review = self.repository.show(id).await?;

        match review {
            Some(review) => Ok(review),
            None => Err(AppError::NotFound("Review not found".to_string())),
        }
    }

    /**
     * Get one review (public version).
     */
    pub async fn get_one_public(&self, id: i64) -> Result<AdminPublicReview, AppError> {
        let review = self.get_one(id).await?;

        Ok(review.into_public())
    }

    pub async fn update_status(
        &self,
        cmd: UpdateReviewStatusCommand,
        id: i64,
    ) -> Result<ReviewApprovalStatus, AppError> {
        self.get_one(id).await?;

        self.repository
            .update_status(self.repository.get_pool(), &cmd.status, id)
            .await?;

        Ok(cmd.status)
    }

    pub async fn delete(&self, id: i64) -> Result<u64, AppError> {
        self.get_one(id).await?;
        self.repository.delete(self.repository.get_pool(), id).await
    }
}
