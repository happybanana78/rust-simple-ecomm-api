use crate::admin::products::permission::ProductScope;
use crate::admin::products::videos::handler;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use actix_web::web::{delete, get, post, resource};
use std::sync::Arc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/videos")
            .service(
                resource("/upload")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Create))))
                    .route(post().to(handler::upload)),
            )
            .service(
                resource("/delete/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Delete))))
                    .route(delete().to(handler::delete)),
            )
            .service(
                resource("/{id}/stream")
                    .wrap(AuthMiddleware::new(Some(Arc::new(ProductScope::Read))))
                    .route(get().to(handler::stream)),
            ),
    );
}
