use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::PgPool;
use super::{dto, service};
use validator::Validate;
use crate::errors::response::ErrorResponse;

#[get("/products")]
pub async fn index(pool: web::Data<PgPool>) -> impl Responder {
    match service::index(&pool).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/products/{id}")]
pub async fn show(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder {
    match service::show(&pool, id.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {message: "Product not found".to_string()}),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/products")]
pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<dto::CreateProductDTO>
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    match service::create(&pool, body.0).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[put("/products/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    body: web::Json<dto::UpdateProductDTO>,
    id: web::Path<i32>
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    match service::update(&pool, body.0, id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[delete("/products/{id}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder {
    match service::delete(&pool, id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
