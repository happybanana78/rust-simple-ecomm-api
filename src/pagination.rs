use serde::{Deserialize, Serialize};

pub struct DataCollection<T> {
    pub data: Vec<T>,
}

impl<T> DataCollection<T> {
    pub fn new(data: Vec<T>) -> Self {
        DataCollection { data }
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

#[derive(Debug, Deserialize)]
pub struct PaginatedDataCollection<T> {
    pub data: Vec<T>,
    pub pagination: Paginate,
}

impl<T> PaginatedDataCollection<T> {
    pub fn new(data: Vec<T>, pagination: Paginate) -> Self {
        PaginatedDataCollection { data, pagination }
    }
}
