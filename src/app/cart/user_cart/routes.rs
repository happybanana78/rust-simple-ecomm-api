use crate::app::cart::user_cart::handler;
use crate::middlewares::auth::AuthMiddleware;
use actix_web::web::{ServiceConfig, delete, get, post, put, resource, scope};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(AuthMiddleware::new(None))
            .service(resource("/get").route(get().to(handler::get_user_cart)))
            .service(resource("/add").route(post().to(handler::add_item)))
            .service(resource("/update").route(put().to(handler::update_item)))
            .service(resource("/remove").route(delete().to(handler::remove_item))),
    );
}
