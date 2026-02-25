use crate::errors::error::AppError;
use std::collections::HashMap;

pub fn validate_slug(slug: &str) -> Result<(), AppError> {
    if slug.contains(" ") {
        Err(AppError::Conflict(
            "Slug cannot contain empty spaces".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn validate_target_index(target_index: Option<i32>) -> Result<usize, AppError> {
    let target_index = target_index.ok_or_else(|| {
        let mut error = HashMap::new();

        error.insert(
            "target_index".to_string(),
            vec!["field is required".to_string()],
        );

        AppError::ValidationSingle(error)
    })?;

    if target_index < 0 {
        let mut error = HashMap::new();

        error.insert(
            "target_index".to_string(),
            vec!["must be at least 0".to_string()],
        );

        return Err(AppError::ValidationSingle(error));
    }

    Ok(target_index as usize)
}
