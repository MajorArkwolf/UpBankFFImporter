use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSearchRequest {
    #[serde(default)]
    pub data: Vec<TransactionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
    pub attributes: Attributes,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    pub created_at: String,
    pub updated_at: String,
    pub user: String,
    pub group_title: Option<String>,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub user: String,
    pub transaction_journal_id: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub date: String,
    pub order: Option<i32>,
    pub currency_id: Option<String>,
    pub currency_code: Option<String>,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimal_places: i64,
    pub foreign_currency_id: Option<String>,
    pub foreign_currency_code: Option<String>,
    pub foreign_currency_symbol: Option<String>,
    pub foreign_currency_decimal_places: Option<i32>,
    pub amount: String,
    pub foreign_amount: Option<String>,
    pub description: String,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
    pub source_iban: Option<String>,
    pub source_type: String,
    pub destination_id: Option<String>,
    pub destination_name: Option<String>,
    pub destination_iban: Option<String>,
    pub destination_type: String,
    pub budget_id: Option<String>,
    pub budget_name: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub bill_id: Option<String>,
    pub bill_name: Option<String>,
    pub reconciled: bool,
    pub notes: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub internal_reference: Option<String>,
    pub external_id: Option<String>,
    pub external_url: Option<String>,
    pub original_source: Option<String>,
    pub recurrence_id: Option<i32>,
    pub recurrence_total: Option<i32>,
    pub recurrence_count: Option<i32>,
    pub bunq_payment_id: Option<String>,
    pub import_hash_v2: Option<String>,
    pub sepa_cc: Option<String>,
    pub sepa_ct_op: Option<String>,
    pub sepa_ct_id: Option<String>,
    pub sepa_db: Option<String>,
    pub sepa_country: Option<String>,
    pub sepa_ep: Option<String>,
    pub sepa_ci: Option<String>,
    pub sepa_batch_id: Option<String>,
    pub interest_date: Option<String>,
    pub book_date: Option<String>,
    pub process_date: Option<String>,
    pub due_date: Option<String>,
    pub payment_date: Option<String>,
    pub invoice_date: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub zoom_level: Option<i64>,
    pub has_attachments: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "0")]
    pub the_0: The0,
    #[serde(rename = "self")]
    pub links_self: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct The0 {
    pub rel: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInsertRequest {
    pub error_if_duplicate_hash: bool,
    pub apply_rules: bool,
    pub fire_webhooks: bool,
    pub group_title: String,
    #[serde(default)]
    pub transactions: Vec<TransactionPayload>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TransactionPayload {
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub date: String,
    pub amount: String,
    pub description: String,
    pub order: Option<i32>,
    pub currency_id: Option<String>,
    pub currency_code: Option<String>,
    pub foreign_amount: Option<String>,
    pub foreign_currency_id: Option<String>,
    pub foreign_currency_code: Option<String>,
    pub budget_id: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
    pub destination_id: Option<String>,
    pub destination_name: Option<String>,
    pub reconciled: bool,
    pub piggy_bank_id: Option<i64>,
    pub piggy_bank_name: Option<String>,
    pub bill_id: Option<String>,
    pub bill_name: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub internal_reference: Option<String>,
    pub external_id: Option<String>,
    pub external_url: Option<String>,
    pub bunq_payment_id: Option<String>,
    pub sepa_cc: Option<String>,
    pub sepa_ct_op: Option<String>,
    pub sepa_ct_id: Option<String>,
    pub sepa_db: Option<String>,
    pub sepa_country: Option<String>,
    pub sepa_ep: Option<String>,
    pub sepa_ci: Option<String>,
    pub sepa_batch_id: Option<String>,
    pub interest_date: Option<String>,
    pub book_date: Option<String>,
    pub process_date: Option<String>,
    pub due_date: Option<String>,
    pub payment_date: Option<String>,
    pub invoice_date: Option<String>,
}
