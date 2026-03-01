use crate::app::cart::guest_cart::handler;
use crate::middlewares::guest::GuestMiddleware;
use actix_web::web::{ServiceConfig, delete, get, post, put, resource, scope};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guest")
            .wrap(GuestMiddleware)
            .service(resource("/get").route(get().to(handler::get_guest_cart)))
            .service(resource("/add").route(post().to(handler::add_item)))
            .service(resource("/update").route(put().to(handler::update_item)))
            .service(resource("/remove").route(delete().to(handler::remove_item))),
    );
}
