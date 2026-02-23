use super::handler;
use actix_web::web;
use actix_web::web::{get, resource};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .service(resource("/list").route(get().to(handler::index)))
            .service(resource("/get/{slug}").route(get().to(handler::show))),
    );
}
