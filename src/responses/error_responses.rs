use crate::pagination::Paginate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
            errors: None,
        }
    }
}

#[derive(Serialize)]
pub struct SuccessResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub data: Option<T>,
    pub meta: Option<Paginate>,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            message: "success".to_string(),
            data: Some(data),
            meta: None,
        }
    }

    pub fn ok_with_pagination(data: T, pagination: Paginate) -> Self {
        Self {
            message: "success".to_string(),
            data: Some(data),
            meta: Some(pagination),
        }
    }

    pub fn empty() -> Self {
        Self {
            message: "success".to_string(),
            data: None,
            meta: None,
        }
    }
}
