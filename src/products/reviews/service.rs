use crate::admin::products::service::AdminProductService;
use crate::admin::users::service::AdminUserService;
use crate::errors::error::AppError;
use crate::products::reviews::dto::{CreateProductReviewCommand, PublicProductReview};
use crate::products::reviews::repository::ProductReviewRepository;
use crate::products::reviews::traits::IntoPublic;
use crate::traits::IsRepository;
use sqlx::PgPool;

pub struct ProductReviewService {
    repository: ProductReviewRepository,
    admin_product_service: AdminProductService,
    admin_user_service: AdminUserService,
}

impl ProductReviewService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: ProductReviewRepository::new(pool.clone()),
            admin_product_service: AdminProductService::new(pool.clone()),
            admin_user_service: AdminUserService::new(pool),
        }
    }

    pub async fn create(
        &self,
        cmd: CreateProductReviewCommand,
    ) -> Result<PublicProductReview, AppError> {
        self.admin_product_service.get_one(cmd.product_id).await?;

        if let Some(user_id) = cmd.user_id {
            self.admin_user_service.get_one_safe(user_id).await?;
        }

        let review = self
            .repository
            .create(self.repository.get_pool(), cmd)
            .await?;

        Ok(review.into_public())
    }
}
