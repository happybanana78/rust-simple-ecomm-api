use crate::auth::traits::Scope;

pub enum ProductScope {
    Create,
    Read,
    Update,
    Delete,
    List,
}

impl Scope for ProductScope {
    fn as_str(&self) -> &'static str {
        match self {
            ProductScope::Create => "products:create",
            ProductScope::Read => "products:read",
            ProductScope::Update => "products:update",
            ProductScope::Delete => "products:delete",
            ProductScope::List => "products:list",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            ProductScope::Create,
            ProductScope::Read,
            ProductScope::Update,
            ProductScope::Delete,
            ProductScope::List,
        ]
    }
}
