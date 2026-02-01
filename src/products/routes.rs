use actix_web::web;
use sqlx::PgPool;
use crate::middlewares::auth::AuthMiddleware;
use super::handler;

pub fn routes(cfg: &mut web::ServiceConfig, pool: PgPool) {
    cfg
        .service(
            web::scope("/products")
                .service(handler::index)
                .service(handler::show)
        )

        .service(
            web::scope("/admin/products")
                .wrap(AuthMiddleware::new(pool))
                .service(handler::create)
                .service(handler::update)
                .service(handler::delete)
        );
}
