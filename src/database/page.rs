use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: i64,
    pub page_size: i64,
    pub has_more: bool,
}   

pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub request_params: PageRequest,   
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}