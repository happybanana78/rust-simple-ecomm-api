use super::handler;
use crate::auth::traits::Scope;
use crate::products::permission::ProductScope;
use actix_web::web;
use actix_web::web::{get, resource};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(
                resource("")
                    .app_data(web::Data::new(
                        Box::new(ProductScope::List) as Box<dyn Scope>
                    ))
                    .route(get().to(handler::index)),
            )
            .service(
                resource("/{id}")
                    .app_data(web::Data::new(
                        Box::new(ProductScope::Read) as Box<dyn Scope>
                    ))
                    .route(get().to(handler::show)),
            ),
    );
}
