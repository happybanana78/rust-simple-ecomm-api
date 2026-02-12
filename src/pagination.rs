pub struct DataCollection<T> {
    pub data: Vec<T>,
}

impl<T> DataCollection<T> {
    pub fn new(data: Vec<T>) -> Self {
        DataCollection { data }
    }
}

pub struct Paginate {
    pub limit: i64,
    pub offset: i64,
}

impl Paginate {
    pub fn new(limit: i64, offset: i64) -> Self {
        Paginate { limit, offset }
    }

    pub fn default() -> Self {
        Paginate {
            limit: 10,
            offset: 0,
        }
    }
}

pub struct PaginatedDataCollection<T> {
    pub data: Vec<T>,
    pub pagination: Paginate,
}

impl<T> PaginatedDataCollection<T> {
    pub fn new(data: Vec<T>, pagination: Paginate) -> Self {
        PaginatedDataCollection { data, pagination }
    }
}
