use crate::admin::reviews::handler;
use crate::admin::reviews::permission::ProductReviewScope;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use actix_web::web::{delete, get, put, resource};
use std::sync::Arc;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reviews")
            .service(
                resource("/list")
                    .wrap(AuthMiddleware::new(Some(Arc::new(
                        ProductReviewScope::List,
                    ))))
                    .route(get().to(handler::index)),
            )
            .service(
                resource("/{review_id}/get")
                    .wrap(AuthMiddleware::new(Some(Arc::new(
                        ProductReviewScope::Read,
                    ))))
                    .route(get().to(handler::show)),
            )
            .service(
                resource("/{review_id}/update-status")
                    .wrap(AuthMiddleware::new(Some(Arc::new(
                        ProductReviewScope::Update,
                    ))))
                    .route(put().to(handler::update_status)),
            )
            .service(
                resource("/{review_id}/delete")
                    .wrap(AuthMiddleware::new(Some(Arc::new(
                        ProductReviewScope::Delete,
                    ))))
                    .route(delete().to(handler::delete)),
            ),
    );
}
