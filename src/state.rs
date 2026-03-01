use crate::admin::categories::service::AdminCategoryService;
use crate::admin::products::images::service::AdminProductImageService;
use crate::admin::products::service::AdminProductService;
use crate::admin::products::videos::service::AdminProductVideoService;
use crate::admin::reviews::service::AdminReviewService;
use crate::admin::users::service::AdminUserService;
use crate::app::cart::cart_items::service::CartItemsService;
use crate::app::cart::guest_cart::service::GuestCartService;
use crate::app::cart::user_cart::service::UserCartService;
use crate::app::categories::service::CategoryService;
use crate::app::products::reviews::service::ProductReviewService;
use crate::app::products::service::ProductService;
use crate::app::users::service::UserService;
use crate::auth::service::AuthService;
use crate::utils::storage::LocalStorage;
use sqlx::PgPool;

pub struct AppState {
    pub auth_service: AuthService,
    pub category_service: CategoryService,
    pub product_service: ProductService,
    pub user_cart_service: UserCartService,
    pub guest_cart_service: GuestCartService,
    pub cart_items_service: CartItemsService,
    pub user_service: UserService,
    pub reviews_service: ProductReviewService,

    // admin services
    pub admin_product_service: AdminProductService,
    pub admin_product_images_service: AdminProductImageService,
    pub admin_product_videos_service: AdminProductVideoService,
    pub admin_category_service: AdminCategoryService,
    pub admin_reviews_service: AdminReviewService,
    pub admin_user_service: AdminUserService,

    // storage
    pub local_storage: LocalStorage,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            auth_service: AuthService::new(pool.clone()),
            product_service: ProductService::new(pool.clone()),
            category_service: CategoryService::new(pool.clone()),
            user_cart_service: UserCartService::new(pool.clone()),
            guest_cart_service: GuestCartService::new(pool.clone()),
            cart_items_service: CartItemsService::new(pool.clone()),
            user_service: UserService::new(pool.clone()),
            reviews_service: ProductReviewService::new(pool.clone()),

            // admin services
            admin_product_service: AdminProductService::new(pool.clone()),
            admin_product_images_service: AdminProductImageService::new(pool.clone()),
            admin_product_videos_service: AdminProductVideoService::new(pool.clone()),
            admin_category_service: AdminCategoryService::new(pool.clone()),
            admin_reviews_service: AdminReviewService::new(pool.clone()),
            admin_user_service: AdminUserService::new(pool),

            // storage
            local_storage: LocalStorage::new("public/uploads".to_string()),
        }
    }
}
