use serde::{Deserialize, Serialize};

use super::{
    general::{self, MoneyObject},
    pagination::Pagination,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsResponse {
    pub data: Vec<Account>,
    pub links: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "type")]
    pub account_type: String,
    pub id: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
    pub links: general::Links,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "ownershipType")]
    pub ownership_type: String,
    pub balance: MoneyObject,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub transactions: Transactions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions {
    pub links: TransactionsLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsLinks {
    pub related: String,
}
