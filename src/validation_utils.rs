use crate::errors::error::AppError;

pub fn validate_slug(slug: &str) -> Result<(), AppError> {
    let split = slug.split("").into_iter().any(|s| s.is_empty());
    if split {
        Err(AppError::Internal(
            "Slug cannot contain empty spaces".to_string(),
        ))
    } else {
        Ok(())
    }
}
