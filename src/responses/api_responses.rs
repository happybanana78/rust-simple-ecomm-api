use crate::pagination::Paginate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LocalApiResponse<T> {
    data: T,
}

impl<T> LocalApiResponse<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

#[derive(Deserialize, Debug)]
pub struct LocalApiPaginatedResponse<T> {
    data: T,
    meta: Paginate,
}

impl<T> LocalApiPaginatedResponse<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }
}
