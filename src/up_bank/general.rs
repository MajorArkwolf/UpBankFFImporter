use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinksRelated {
    pub related: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename = "type")]
    pub dat_type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tags {
    pub data: Vec<Data>,
    pub links: Option<Links>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoneyObject {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: String,
    #[serde(rename = "valueInBaseUnits")]
    pub value_in_base_units: i64,
}
