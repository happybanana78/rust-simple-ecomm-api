use crate::utils::traits::HasId;
use serde::{Deserialize, Serialize};

pub struct DataCollection<T> {
    pub data: Vec<T>,
}

impl<T> DataCollection<T>
where
    T: HasId,
{
    pub fn new(data: Vec<T>) -> Self {
        DataCollection { data }
    }

    pub fn extract_ids(&self) -> Vec<i64> {
        self.data.iter().map(|item| item.get_id()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginate {
    pub limit: i64,
    pub page: i64,
}

impl Paginate {
    pub fn new(limit: i64, page: i64) -> Self {
        Paginate { limit, page }
    }

    pub fn get_offset(&self) -> i64 {
        (self.page - 1) * self.limit
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaginatedDataCollection<T> {
    pub data: Vec<T>,
    pub pagination: Paginate,
}

impl<T> PaginatedDataCollection<T>
where
    T: HasId + Clone,
{
    pub fn new(data: Vec<T>, pagination: Paginate) -> Self {
        PaginatedDataCollection { data, pagination }
    }

    pub fn get_data(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn get_pagination(&self) -> Paginate {
        self.pagination.clone()
    }

    pub fn extract_ids(&self) -> Vec<i64> {
        self.data.iter().map(|item| item.get_id()).collect()
    }
}
