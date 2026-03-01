use crate::auth::dto::AuthUserId;
use crate::errors::error::AppError;
use actix_web::{HttpMessage, HttpRequest};

pub fn extract_auth_user_id(req: &HttpRequest) -> Result<i64, AppError> {
    req.extensions()
        .get::<AuthUserId>()
        .map(|id| id.0)
        .ok_or(AppError::Unauthorized("unauthorized".to_string()))
}
