use super::general::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoriesResponse {
    pub data: Vec<Categorie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Categorie {
    #[serde(rename = "type")]
    pub categorie_type: String,
    pub id: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
    pub links: general::Links,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub parent: CategorieLink,
    pub children: ChildCategories,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    pub links: Option<general::LinksRelated>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategorieLink {
    pub data: Option<general::Data>,
    pub links: Option<general::LinksRelated>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildCategories {
    pub data: Vec<general::Data>,
    pub links: Option<general::LinksRelated>,
}
