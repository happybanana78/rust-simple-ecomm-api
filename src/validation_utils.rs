use crate::errors::error::AppError;

pub fn validate_slug(slug: &str) -> Result<(), AppError> {
    if slug.contains(" ") {
        Err(AppError::Conflict(
            "Slug cannot contain empty spaces".to_string(),
        ))
    } else {
        Ok(())
    }
}
