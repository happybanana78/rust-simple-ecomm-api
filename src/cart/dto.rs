use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
#[validate(schema(function = "check_validity"))]
pub struct GetCartDto {
    user_id: Option<i64>,
    user_hash: Option<String>,
}

/**
* One of the two fields must be present
*/
fn check_validity(dto: &GetCartDto) -> Result<(), ValidationError> {
    match (&dto.user_id, &dto.user_hash) {
        (None, None) => Err(ValidationError::new("user_id_or_hash_required")),
        _ => Ok(())
    }
}

pub struct CartByUserCommand {
    pub user_id: i64,
}

pub struct CartByHashCommand {
    pub user_hash: String,
}

pub enum GetCartCommand {
    ByUser(CartByUserCommand),
    ByHash(CartByHashCommand),
}

impl GetCartDto {
    pub fn into_command(self) -> GetCartCommand {
        match (self.user_id, self.user_hash) {
            (Some(user_id), None) => GetCartCommand::ByUser(
                CartByUserCommand { user_id }
            ),
            (None, Some(user_hash)) => GetCartCommand::ByHash(
                CartByHashCommand { user_hash }
            ),
            (Some(user_id), Some(_)) => GetCartCommand::ByUser(
                CartByUserCommand { user_id }
            ),
            (None, None) => unreachable!("DTO already validated"),
        }
    }
}
