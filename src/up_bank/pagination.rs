use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub prev: Option<String>,
    pub next: Option<String>,
}
