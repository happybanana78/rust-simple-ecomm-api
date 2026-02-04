use crate::cart::cart_items::handler;
use actix_web::web::{ServiceConfig, scope};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/items")
            .service(handler::add_item)
            .service(handler::remove_item)
            .service(handler::update_item),
    );
}
