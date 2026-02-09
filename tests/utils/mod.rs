use actix_test::TestServer;
use actix_web::{App, web};
use dotenvy::from_filename;
use ecomm::auth::dto::LoginDTO;
use ecomm::auth::routes::routes as auth_routes;
use ecomm::cart::routes::routes as cart_routes;
use ecomm::products::routes::routes as products_routes;
use ecomm::state::AppState;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub struct TestDatabase {
    pub pool: PgPool,
    pub name: String,
    admin_pool: PgPool,
}

impl TestDatabase {
    pub async fn new() -> Self {
        from_filename(".env.test").ok();
        env_logger::try_init().ok();

        let test_db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let admin_db_url = env::var("ADMIN_DATABASE_URL").expect("ADMIN_DATABASE_URL must be set");
        let test_db_name = env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME must be set");

        let admin_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&admin_db_url)
            .await
            .expect("Failed to connect to admin database");

        sqlx::query(&format!("CREATE DATABASE \"{}\"", test_db_name))
            .execute(&admin_pool)
            .await
            .expect("Failed to create test database");

        let test_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&test_db_url)
            .await
            .expect("Failed to connect to test database");

        sqlx::migrate!("./migrations")
            .run(&test_pool)
            .await
            .expect("Failed to run migrations");

        Self {
            pool: test_pool,
            name: test_db_name,
            admin_pool,
        }
    }

    pub async fn cleanup(self) {
        self.pool.close().await;

        let _ = sqlx::query(&format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            self.name
        ))
        .execute(&self.admin_pool)
        .await;

        let _ = sqlx::query(&format!("DROP DATABASE IF EXISTS \"{}\"", self.name))
            .execute(&self.admin_pool)
            .await;

        self.admin_pool.close().await;
    }
}

pub fn create_test_server(pool: PgPool) -> TestServer {
    actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(pool.clone())))
            .configure(auth_routes)
            .configure(products_routes)
            .configure(cart_routes)
    })
}

pub async fn auto_login(srv: &TestServer, email: String) -> String {
    let payload = LoginDTO {
        email: Some(email),
        password: Some("123456".to_string()),
    };

    let mut res = srv.post("/auth/login").send_json(&payload).await.unwrap();

    let body = res.json::<serde_json::Value>().await.unwrap();

    body.get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .trim_matches('"')
        .to_string()
}

pub async fn seed_roles(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO roles (id, name) VALUES
         (1, 'admin'),
         (2, 'user')
         ON CONFLICT (id) DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed user test data");
}

pub async fn seed_users(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO users (id, email, username, password) VALUES
         (1, 'test1@test.com', 'Test1', '$argon2id$v=19$m=19456,t=2,p=1$cvMtKFBV7DMHHq7DLLCDAg$JbZ3kqb47wjU5IxeZmeea/6yYIC8Yz6Xqe1KwgWwroc'),
         (2, 'test2@test.com', 'Test2', '$argon2id$v=19$m=19456,t=2,p=1$cvMtKFBV7DMHHq7DLLCDAg$JbZ3kqb47wjU5IxeZmeea/6yYIC8Yz6Xqe1KwgWwroc')
         ON CONFLICT (id) DO NOTHING;"
    )
        .execute(pool)
        .await
        .expect("Failed to seed user test data");

    sqlx::query!(
        "INSERT INTO user_has_roles (user_id, role_id) VALUES (1, 2), (2, 2)
         ON CONFLICT DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed roles for user test data");
}

pub async fn seed_products(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO products (id, name, price, quantity) VALUES
         (1, 'Test Product 1', 10.99, 10),
         (2, 'Test Product 2', 20.99, 12)
         ON CONFLICT (id) DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed product test data");
}
