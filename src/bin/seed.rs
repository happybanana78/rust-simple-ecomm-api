use dotenvy::from_filename;
use ecomm::db::seeders;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    from_filename(".env.dev").ok();
    env_logger::try_init().ok();

    println!("Starting seeding...");

    let database_url = env::var("DEV_DATABASE_URL").expect("DEV_DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    seeders::seed_all(&pool).await?;
    println!("Seeding done!");
    Ok(())
}
