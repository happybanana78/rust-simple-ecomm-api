use crate::auth::traits::Scope;

pub enum CategoryScope {
    Create,
    Read,
    Update,
    Delete,
    List,
}

impl Scope for CategoryScope {
    fn as_str(&self) -> &'static str {
        match self {
            CategoryScope::Create => "categories:create",
            CategoryScope::Read => "categories:read",
            CategoryScope::Update => "categories:update",
            CategoryScope::Delete => "categories:delete",
            CategoryScope::List => "categories:list",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            CategoryScope::Create,
            CategoryScope::Read,
            CategoryScope::Update,
            CategoryScope::Delete,
            CategoryScope::List,
        ]
    }
}
