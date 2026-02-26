use crate::admin::categories::service::AdminCategoryService;
use crate::admin::products::images::service::AdminProductImageService;
use crate::admin::products::service::AdminProductService;
use crate::admin::products::videos::service::AdminProductVideoService;
use crate::admin::reviews::service::AdminReviewService;
use crate::auth::service::AuthService;
use crate::cart::cart_items::service::CartItemsService;
use crate::cart::guest_cart::service::GuestCartService;
use crate::cart::user_cart::service::UserCartService;
use crate::categories::service::CategoryService;
use crate::products::service::ProductService;
use crate::storage::LocalStorage;
use crate::users::service::UserService;
use sqlx::PgPool;

pub struct AppState {
    pub auth_service: AuthService,
    pub category_service: CategoryService,
    pub product_service: ProductService,
    pub user_cart_service: UserCartService,
    pub guest_cart_service: GuestCartService,
    pub cart_items_service: CartItemsService,
    pub user_service: UserService,

    // admin services
    pub admin_product_service: AdminProductService,
    pub admin_product_images_service: AdminProductImageService,
    pub admin_product_videos_service: AdminProductVideoService,
    pub admin_category_service: AdminCategoryService,
    pub admin_reviews_service: AdminReviewService,

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

            // admin services
            admin_product_service: AdminProductService::new(pool.clone()),
            admin_product_images_service: AdminProductImageService::new(pool.clone()),
            admin_product_videos_service: AdminProductVideoService::new(pool.clone()),
            admin_category_service: AdminCategoryService::new(pool.clone()),
            admin_reviews_service: AdminReviewService::new(pool),

            // storage
            local_storage: LocalStorage::new("public/uploads".to_string()),
        }
    }
}
