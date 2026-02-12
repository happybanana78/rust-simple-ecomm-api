mod admin;
mod auth;
mod cart;
mod errors;
mod middlewares;
mod pagination;
mod products;
mod responses;
mod roles;
mod state;
mod traits;
mod users;

use actix_web::{App, HttpServer, web};
use dotenvy::from_filename;
use sqlx::PgPool;
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    from_filename(".env.dev").ok();
    env_logger::try_init().ok();

    let database_url = env::var("DEV_DATABASE_URL").expect("DEV_DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(pool.clone())))
            .configure(auth::routes::routes)
            .configure(admin::routes::routes)
            .configure(products::routes::routes)
            .configure(cart::routes::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// TODO: add orders and stock handling
