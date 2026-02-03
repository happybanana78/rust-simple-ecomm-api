use actix_web::web::{scope, ServiceConfig};
use crate::cart::cart_items::handler;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(
            scope("/items")
                .service(handler::add_item)
                .service(handler::remove_item)
                .service(handler::update_item)
        );
}
