use crate::cart::cart_items::routes::routes as cart_items_routes;
use crate::cart::handler;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web::{ServiceConfig, post, resource, scope};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/cart").configure(cart_items_routes).service(
            resource("/get")
                .wrap(AuthMiddleware::new(None))
                .route(post().to(handler::get_user_cart)),
        ),
    );
}
