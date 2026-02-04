use super::handler;
use crate::products::permission::ProductScope;
use actix_web::web;
use actix_web::web::{get, resource};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(
                resource("")
                    .app_data(web::Data::new(ProductScope::List))
                    .route(get().to(handler::index)),
            )
            .service(
                resource("/{id}")
                    .app_data(web::Data::new(ProductScope::Read))
                    .route(get().to(handler::show)),
            ),
    );
}
