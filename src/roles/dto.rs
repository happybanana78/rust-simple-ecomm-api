use crate::admin::categories::permission::CategoryScope;
use crate::admin::products::permission::ProductScope;
use crate::auth::traits::Scope;
use crate::errors::error::AppError;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Role {
    pub name: String,
}

#[derive(Debug)]
pub enum RoleEnum {
    Admin,
    User,
}

impl RoleEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleEnum::Admin => "admin",
            RoleEnum::User => "user",
        }
    }

    pub fn get_scopes(&self) -> HashSet<String> {
        match self {
            RoleEnum::Admin => {
                let mut scopes = HashSet::new();

                // product scopes
                scopes.extend(ProductScope::all().iter().map(|s| s.as_str().to_string()));

                // category scopes
                scopes.extend(CategoryScope::all().iter().map(|s| s.as_str().to_string()));

                scopes
            }
            RoleEnum::User => HashSet::from([]),
        }
    }
}

impl FromStr for RoleEnum {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleEnum::Admin),
            "user" => Ok(RoleEnum::User),
            _ => Err(AppError::Internal(format!("invalid role: {}", s))),
        }
    }
}
