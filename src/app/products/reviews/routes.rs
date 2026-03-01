use crate::app::products::reviews::handler;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use actix_web::web::{post, resource};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{product_id}/reviews")
            .service(
                resource("/create-user")
                    .wrap(AuthMiddleware::new(None))
                    .route(post().to(handler::create_user)),
            )
            .service(resource("/create-guest").route(post().to(handler::create_guest))),
    );
}
