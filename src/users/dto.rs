use crate::roles::dto::RoleEnum;

pub struct UserScopes {
    pub scopes: Vec<String>,
}

impl UserScopes {
    pub fn by_role(role: RoleEnum) -> Self {
        match role {
            RoleEnum::Admin => Self {
                scopes: vec![
                    "create-user".to_string(),
                    "read-user".to_string(),
                    "update-user".to_string(),
                    "delete-user".to_string(),
                    "list-users".to_string(),
                ],
            },
            RoleEnum::User => Self {
                scopes: vec!["read-user".to_string()],
            },
        }
    }
}
