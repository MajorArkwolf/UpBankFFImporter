use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub prev: Option<String>,
    pub next: Option<String>,
}
