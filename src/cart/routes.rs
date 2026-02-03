use actix_web::web::{scope, ServiceConfig};
use crate::cart::cart_items::routes::routes as cart_items_routes;
use crate::products::handler;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(
            scope("/cart")
                .configure(cart_items_routes)
                .service(handler::show)
        );
}
