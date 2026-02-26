use crate::auth::traits::Scope;

pub enum ProductReviewScope {
    Read,
    Update,
    Delete,
    List,
}

impl Scope for ProductReviewScope {
    fn as_str(&self) -> &'static str {
        match self {
            ProductReviewScope::Read => "products:read",
            ProductReviewScope::Update => "products:update",
            ProductReviewScope::Delete => "products:delete",
            ProductReviewScope::List => "products:list",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            ProductReviewScope::Read,
            ProductReviewScope::Update,
            ProductReviewScope::Delete,
            ProductReviewScope::List,
        ]
    }
}
