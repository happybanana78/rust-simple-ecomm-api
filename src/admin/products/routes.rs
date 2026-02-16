use super::handler;
use crate::admin::products::permission::ProductScope;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use actix_web::web::{delete, get, post, put, resource};
use std::sync::Arc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(
                resource("/list")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::List))))
                    .route(get().to(handler::index)),
            )
            .service(
                resource("/get/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Read))))
                    .route(get().to(handler::show)),
            )
            .service(
                resource("/create")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Create))))
                    .route(post().to(handler::create)),
            )
            .service(
                resource("/update/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Update))))
                    .route(put().to(handler::update)),
            )
            .service(
                resource("/delete/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Delete))))
                    .route(delete().to(handler::delete)),
            ),
    );
}
