use crate::admin::products::images::handler;
use crate::admin::products::permission::ProductScope;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use actix_web::web::{delete, post, resource};
use std::sync::Arc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/images")
            .service(
                resource("/upload")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Create))))
                    .route(post().to(handler::upload)),
            )
            .service(
                resource("/delete/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Delete))))
                    .route(delete().to(handler::delete)),
            ),
    );
}
