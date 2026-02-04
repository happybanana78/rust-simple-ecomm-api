use super::handler;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(handler::index)
            .service(handler::show)
            .service(handler::create)
            .service(handler::update)
            .service(handler::delete),
    );
}
