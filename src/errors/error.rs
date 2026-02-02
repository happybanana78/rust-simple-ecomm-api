use std::collections::HashMap;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use thiserror::Error;
use validator::ValidationErrors;
use crate::errors::response::ErrorResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation error")]
    Validation(ValidationErrors),

    #[error("unauthorized: {0:?}")]
    Unauthorized(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("resource not found")]
    NotFound(String),

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Validation(errors) => {
                HttpResponse::UnprocessableEntity().json(ErrorResponse {
                    message: "validation failed".to_string(),
                    errors: Some(extract_validation_errors(errors))
                })
            }

            AppError::Unauthorized(err) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    message: err.clone(),
                    errors: None,
                })
            }

            AppError::Internal(err) => {
                error!("Internal error: {}", err);

                #[cfg(debug_assertions)]
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    message: err.clone(),
                    errors: None,
                });

                #[cfg(not(debug_assertions))]
                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: "internal server error".to_string()
                })
            }

            AppError::Conflict(err) => {
                error!("Conflict error: {}", err);

                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: format!("{}", err),
                    errors: None,
                })
            }

            AppError::NotFound(err) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    message: format!("{}", err),
                    errors: None,
                })
            }

            AppError::Database(err) => {
                error!("Database error: {}", err);

                #[cfg(debug_assertions)]
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    message: err.to_string(),
                    errors: None,
                });

                #[cfg(not(debug_assertions))]
                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: "database error".to_string()
                })
            }
        }
    }
}

fn extract_validation_errors(errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for (field, errs) in errors.field_errors() {
        let messages = errs
            .iter()
            .map(|err| {
                err.code.to_string()
            })
            .collect::<Vec<String>>();

        map.insert(field.to_string(), messages);
    }

    map
}
