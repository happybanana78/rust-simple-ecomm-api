use super::handler;
use crate::admin::categories::permission::CategoryScope;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use actix_web::web::{delete, get, post, put, resource};
use std::sync::Arc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .service(
                resource("/list")
                    .wrap(AuthMiddleware::new(Some(Arc::new(CategoryScope::List))))
                    .route(get().to(handler::index)),
            )
            .service(
                resource("/get/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(CategoryScope::Read))))
                    .route(get().to(handler::show)),
            )
            .service(
                resource("/create")
                    .wrap(AuthMiddleware::new(Some(Arc::new(CategoryScope::Create))))
                    .route(post().to(handler::create)),
            )
            .service(
                resource("/update/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(CategoryScope::Update))))
                    .route(put().to(handler::update)),
            )
            .service(
                resource("/delete/{id}")
                    .wrap(AuthMiddleware::new(Some(Arc::new(CategoryScope::Delete))))
                    .route(delete().to(handler::delete)),
            ),
    );
}
