use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub total: i64,
    pub count: i64,
    pub per_page: i64,
    pub current_page: i64,
    pub total_pages: i64,
}
