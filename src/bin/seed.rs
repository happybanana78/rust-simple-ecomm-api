use dotenvy::dotenv;
use ecomm::db::seeders;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    env_logger::init();

    println!("Starting seeding...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    seeders::seed_all(&pool).await?;
    println!("Seeding done!");
    Ok(())
}
