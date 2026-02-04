use super::handler;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::register).service(handler::login);
}
