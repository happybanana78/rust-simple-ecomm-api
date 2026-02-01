use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub errors: Option<HashMap<String, Vec<String>>>,
}
