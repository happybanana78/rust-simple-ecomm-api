use actix_web::web;
use super::handler;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(handler::index)
        .service(handler::show)
        .service(handler::create)
        .service(handler::update)
        .service(handler::delete);
}
