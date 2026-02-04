pub trait Scope: Send + Sync + 'static {
    fn as_str(&self) -> &'static str;
    fn all() -> Vec<Self>
    where
        Self: Sized;
}
