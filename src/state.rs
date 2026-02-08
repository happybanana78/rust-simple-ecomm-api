use crate::products::service::ProductService;
use sqlx::PgPool;

pub struct AppState {
    pub product_service: ProductService,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            product_service: ProductService::new(pool.clone()),
        }
    }
}
