mod admin;
mod auth;
mod cart;
mod categories;
mod errors;
mod middlewares;
mod pagination;
mod products;
mod responses;
mod roles;
mod state;
mod storage;
mod traits;
mod users;
mod validation_utils;

use crate::errors::error::AppError;
use actix_multipart::MultipartError;
use actix_multipart::form::MultipartFormConfig;
use actix_web::error::InternalError;
use actix_web::{App, HttpServer, ResponseError, web};
use dotenvy::from_filename;
use sqlx::PgPool;
use state::AppState;
use std::collections::HashMap;
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
            .app_data(MultipartFormConfig::default().error_handler(|err, _req| {
                let response = match &err {
                    MultipartError::MissingField(field) => {
                        let mut validation_error = HashMap::new();
                        validation_error
                            .insert(field.to_owned(), vec!["Field is required".to_string()]);

                        AppError::ValidationSingle(validation_error)
                    }
                    _ => AppError::Conflict("Invalid multipart payload".to_string()),
                };

                InternalError::from_response(err, response.error_response()).into()
            }))
            .configure(auth::routes::routes)
            .configure(admin::routes::routes)
            .configure(categories::routes::routes)
            .configure(products::routes::routes)
            .configure(cart::routes::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// TODO: handle product reviews
// TODO: handle review replies
// TODO: handle review attachments
