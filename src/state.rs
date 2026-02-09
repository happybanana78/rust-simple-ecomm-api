use crate::auth::service::AuthService;
use crate::cart::cart_items::service::CartItemsService;
use crate::cart::guest_cart::service::GuestCartService;
use crate::cart::user_cart::service::UserCartService;
use crate::products::service::ProductService;
use crate::users::service::UserService;
use sqlx::PgPool;

pub struct AppState {
    pub auth_service: AuthService,
    pub product_service: ProductService,
    pub user_cart_service: UserCartService,
    pub guest_cart_service: GuestCartService,
    pub cart_items_service: CartItemsService,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            auth_service: AuthService::new(pool.clone()),
            product_service: ProductService::new(pool.clone()),
            user_cart_service: UserCartService::new(pool.clone()),
            guest_cart_service: GuestCartService::new(pool.clone()),
            cart_items_service: CartItemsService::new(pool.clone()),
            user_service: UserService::new(pool),
        }
    }
}
