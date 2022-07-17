use super::{
    general::{self},
    pagination::Pagination,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TagsResponse {
    pub data: Vec<Tag>,
    pub links: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    #[serde(rename = "type")]
    pub tag_type: String,
    pub id: String,
    pub relationships: Relationships,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub transactions: Transactions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    pub links: Option<general::LinksRelated>,
}
