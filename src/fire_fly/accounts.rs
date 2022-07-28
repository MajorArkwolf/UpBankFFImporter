use serde::{Deserialize, Serialize};

use super::general::Meta;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountsResponse {
    #[serde(default)]
    pub data: Vec<Account>,
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountResponse {
    pub data: Option<Account>,
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(rename = "type")]
    pub accounts_type: String,
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attributes {
    pub created_at: String,
    pub updated_at: String,
    pub active: bool,
    pub order: Option<i64>,
    pub name: String,
    #[serde(rename = "type")]
    pub attributes_type: String,
    pub account_role: Option<String>,
    pub currency_id: String,
    pub currency_code: String,
    pub currency_symbol: String,
    pub currency_decimal_places: i64,
    pub current_balance: String,
    pub current_balance_date: String,
    pub iban: Option<String>,
    pub bic: Option<String>,
    pub account_number: Option<String>,
    pub opening_balance: String,
    pub current_debt: Option<String>,
    pub opening_balance_date: Option<String>,
    pub virtual_balance: String,
    pub include_net_worth: Option<bool>,
    pub credit_card_type: Option<String>,
    pub monthly_payment_date: Option<String>,
    pub liability_type: Option<String>,
    pub liability_direction: Option<String>,
    pub interest: Option<String>,
    pub interest_period: Option<String>,
    pub notes: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub zoom_level: Option<i64>,
}
