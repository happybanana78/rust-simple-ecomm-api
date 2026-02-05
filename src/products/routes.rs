use super::handler;
use actix_web::web;
use actix_web::web::{get, resource};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(resource("/list").route(get().to(handler::index)))
            .service(resource("/get/{id}").route(get().to(handler::show))),
    );
}
