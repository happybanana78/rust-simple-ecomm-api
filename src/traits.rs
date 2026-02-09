pub trait IsRepository<T> {
    type Repository;

    fn new(pool: T) -> Self::Repository;
}

pub trait HasQuantity {
    fn get_quantity(&self) -> i32;

    fn is_safe_quantity(&self) -> bool {
        if self.get_quantity() < 0 {
            return false;
        }
        true
    }
}
