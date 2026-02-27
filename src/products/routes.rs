use super::handler;
use crate::products::reviews::routes::routes as reviews_routes;
use actix_web::web;
use actix_web::web::{get, resource};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .configure(reviews_routes)
            .service(resource("/list").route(get().to(handler::index)))
            .service(resource("/get/{slug}").route(get().to(handler::show))),
    );
}
