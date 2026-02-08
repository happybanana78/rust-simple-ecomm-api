use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

mod admin;
mod auth;
mod cart;
mod errors;
mod middlewares;
mod products;
mod roles;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
