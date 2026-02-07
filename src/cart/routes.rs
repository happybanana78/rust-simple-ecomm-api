use crate::cart::guest_cart::routes::routes as guest_cart_routes;
use crate::cart::user_cart::routes::routes as user_cart_routes;
use actix_web::web::{ServiceConfig, scope};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/cart")
            .configure(user_cart_routes)
            .configure(guest_cart_routes),
    );
}
