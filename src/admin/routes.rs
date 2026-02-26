use crate::admin::categories::routes as categories_routes;
use crate::admin::products::routes as products_routes;
use crate::admin::reviews::routes as reviews_routes;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            // .wrap(AuthMiddleware::new(pool))
            .configure(products_routes::routes)
            .configure(categories_routes::routes)
            .configure(reviews_routes::routes),
    );
}
