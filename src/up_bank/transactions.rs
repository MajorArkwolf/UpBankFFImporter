use super::{
    general::{self, MoneyObject},
    pagination::Pagination,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub data: Vec<Transaction>,
    pub links: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub datum_type: String,
    pub id: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
    pub links: Option<general::Links>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub status: String, // Type Incorrect (enum)
    #[serde(rename = "rawText")]
    pub raw_text: Option<String>,
    pub description: String,
    pub message: Option<String>,
    #[serde(rename = "isCategorizable")]
    pub is_categorizable: bool,
    #[serde(rename = "holdInfo")]
    pub hold_info: Option<HoldInfoObject>,
    #[serde(rename = "roundUp")]
    pub round_up: Option<RoundUpObject>,
    pub cashback: Option<Cashback>,
    pub amount: MoneyObject,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<MoneyObject>,
    #[serde(rename = "cardPurchaseMethod")]
    pub card_purchase_method: Option<CardPurchaseMethod>,
    #[serde(rename = "settledAt")]
    pub settled_at: Option<String>, // Type Incorrect (datetime)
    #[serde(rename = "createdAt")]
    pub created_at: String, // Type Incorrect (datetime)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub account: AccountID,
    #[serde(rename = "transferAccount")]
    pub transfer_account: AccountID,
    pub category: Category,
    #[serde(rename = "parentCategory")]
    pub parent_category: ParentCategory,
    pub tags: general::Tags,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountID {
    pub data: Option<general::Data>,
    pub links: Option<general::LinksRelated>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub data: Option<general::Data>,
    pub links: Option<CategoryLinks>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParentCategory {
    pub data: Option<general::Data>,
    pub links: Option<general::LinksRelated>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryLinks {
    #[serde(rename = "self")]
    pub links_self: Option<String>,
    pub related: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoldInfoObject {
    pub amount: MoneyObject,
    #[serde(rename = "foreignAmount")]
    pub foreign_amount: Option<MoneyObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundUpObject {
    pub amount: MoneyObject,
    #[serde(rename = "boostPortion")]
    pub boost_portion: Option<MoneyObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cashback {
    pub description: String,
    pub amount: MoneyObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardPurchaseMethod {
    pub method: String,
    #[serde(rename = "cardNumberSuffix")]
    pub card_number_suffix: Option<String>,
}
