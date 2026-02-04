use crate::admin::products::routes as products_routes;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web;
use sqlx::PgPool;

pub fn routes(cfg: &mut web::ServiceConfig, pool: PgPool) {
    cfg.service(
        web::scope("/admin")
            .wrap(AuthMiddleware::new(pool))
            .configure(products_routes::routes),
    );
}
