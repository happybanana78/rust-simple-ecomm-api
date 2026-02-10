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
