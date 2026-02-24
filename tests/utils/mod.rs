use actix_test::TestServer;
use actix_web::{App, web};
use dotenvy::from_filename;
use ecomm::admin::routes::routes as admin_routes;
use ecomm::auth::dto::LoginDTO;
use ecomm::auth::routes::routes as auth_routes;
use ecomm::cart::routes::routes as cart_routes;
use ecomm::categories::routes::routes as category_routes;
use ecomm::products::routes::routes as products_routes;
use ecomm::state::AppState;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use uuid::Uuid;

pub struct TestDatabase {
    pub pool: PgPool,
    pub name: String,
    admin_pool: PgPool,
}

impl TestDatabase {
    pub async fn new() -> Self {
        from_filename(".env.test").ok();
        env_logger::try_init().ok();

        let test_db_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let admin_db_url = env::var("ADMIN_DATABASE_URL").expect("ADMIN_DATABASE_URL must be set");
        let test_db_name = env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME must be set");

        let test_db_name = format!("{}{}", test_db_name, Uuid::new_v4().simple());

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
            .connect(&format!("{}{}", test_db_url, test_db_name))
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

pub struct TestContext {
    pub database: TestDatabase,
    pub srv: TestServer,
    pub auth_token: Option<String>,
}

impl TestContext {
    pub async fn new(user_token_email: Option<String>) -> Self {
        let test_db = TestDatabase::new().await;

        seed_roles(&test_db.pool).await;
        seed_users(&test_db.pool).await;
        seed_categories(&test_db.pool).await;
        seed_products(&test_db.pool).await;
        seed_product_images(&test_db.pool).await;
        seed_product_videos(&test_db.pool).await;

        let test_server = create_test_server(test_db.pool.clone());

        let token = match user_token_email {
            Some(email) => Some(auto_login(&test_server, email).await),
            None => None,
        };

        Self {
            database: test_db,
            srv: test_server,
            auth_token: token,
        }
    }
}

pub struct TestContextNoServer {
    pub database: TestDatabase,
}

impl TestContextNoServer {
    pub async fn new() -> Self {
        let test_db = TestDatabase::new().await;

        seed_roles(&test_db.pool).await;
        seed_users(&test_db.pool).await;
        seed_categories(&test_db.pool).await;
        seed_products(&test_db.pool).await;
        seed_product_images(&test_db.pool).await;
        seed_product_videos(&test_db.pool).await;

        Self { database: test_db }
    }
}

pub fn create_test_server(pool: PgPool) -> TestServer {
    actix_test::start(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(pool.clone())))
            .configure(auth_routes)
            .configure(admin_routes)
            .configure(category_routes)
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

    match body.get("token") {
        Some(token) => token.as_str().unwrap().trim_matches('"').to_string(),
        None => body.get("message").unwrap().to_string(),
    }
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
         (2, 'test2@test.com', 'Test2', '$argon2id$v=19$m=19456,t=2,p=1$cvMtKFBV7DMHHq7DLLCDAg$JbZ3kqb47wjU5IxeZmeea/6yYIC8Yz6Xqe1KwgWwroc'),

         (3, 'admin1@admin.com', 'Admin1', '$argon2id$v=19$m=19456,t=2,p=1$cvMtKFBV7DMHHq7DLLCDAg$JbZ3kqb47wjU5IxeZmeea/6yYIC8Yz6Xqe1KwgWwroc'),
         (4, 'admin2@admin.com', 'Admin2', '$argon2id$v=19$m=19456,t=2,p=1$cvMtKFBV7DMHHq7DLLCDAg$JbZ3kqb47wjU5IxeZmeea/6yYIC8Yz6Xqe1KwgWwroc')
         ON CONFLICT (id) DO NOTHING;"
    )
        .execute(pool)
        .await
        .expect("Failed to seed user test data");

    sqlx::query!(
        "INSERT INTO user_has_roles (user_id, role_id) VALUES (1, 2), (2, 2), (3, 1), (4, 1)
         ON CONFLICT DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed roles for user test data");
}

pub async fn seed_categories(pool: &PgPool) {
    // ids 1 and 2
    sqlx::query!(
        "INSERT INTO categories (name, slug, is_active) VALUES
         ('Test Category 1', 'test-category-1', true),
         ('Test Category 2', 'test-category-2', false)
         ON CONFLICT (id) DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed category test data");
}

pub async fn seed_products(pool: &PgPool) {
    // ids 1 and 2
    sqlx::query!(
        "INSERT INTO products (name, slug, price, quantity, is_active) VALUES
         ('Test Product 1', 'test-product-1', 10.99, 10, true),
         ('Test Product 2', 'test-product-2', 20.99, 12, false)
         ON CONFLICT (id) DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed product test data");
}

pub async fn seed_product_images(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO product_images (product_id, url, alt, is_main, sort) VALUES
         (1, 'public/uploads/p1_example_image1.jpg', 'p1 example image 1', true, 1),
         (1, 'public/uploads/p1_example_image2.jpg', 'p1 example image 2', false, 2),
         (2, 'public/uploads/p2_example_image1.jpg', 'p2 example image 1', true, 1)
         ON CONFLICT (id) DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed product image test data");
}

pub async fn seed_product_videos(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO product_images (product_id, url, alt, is_main, sort) VALUES
         (1, 'public/uploads/p1_example_video1.jpg', 'p1 example video 1', true, 1),
         (1, 'public/uploads/p1_example_video2.jpg', 'p1 example video 2', false, 2),
         (2, 'public/uploads/p2_example_video1.jpg', 'p2 example video 1', true, 1)
         ON CONFLICT (id) DO NOTHING;"
    )
    .execute(pool)
    .await
    .expect("Failed to seed product video test data");
}
