use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

mod products;
mod errors;
mod cart;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(products::routes::routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
