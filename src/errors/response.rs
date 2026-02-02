use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize)]
pub struct SuccessResponse<T>
where T: Serialize
{
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            message: "success".to_string(),
            data: Some(data)
        }
    }
}
