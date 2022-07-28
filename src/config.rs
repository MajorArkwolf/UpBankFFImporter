use crate::fire_fly::FireFly;
use crate::up_bank::UpBank;
use color_eyre::eyre::{eyre, Result};
use super::migrator::account_map::AccountMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::vec;
use tracing::{info, debug};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "upbank_pan")]
    pub up_pan_token: String,
    #[serde(rename = "ff_pan")]
    pub fire_fly_pan_token: String,
    #[serde(rename = "ff_url")]
    pub fire_fly_base_url: String,
    #[serde(rename = "accounts")]
    pub account_mapping: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub async fn get_accounts(&self, up_bank: &UpBank, fire_fly: &FireFly) -> Result<Vec<AccountMap>> {
        let mut account_vector: Vec<AccountMap> = vec![];
        for up_account_id in &self.account_mapping {
            up_bank
            .accounts
            .iter()
            .find(|&x| x.id == *up_account_id)
            .ok_or(eyre!(
                "Up Bank did not have a account id that matched the one supplied"
            ))?;

            let fire_fly_account_id = fire_fly.get_account_by_account_number(&up_account_id).await?.id;
            debug!("Found Up ID {} linked to Firefly ID {}", up_account_id, fire_fly_account_id);
            let new_account = AccountMap::create(up_account_id.clone(), fire_fly_account_id);
            account_vector.push(new_account);
        }

        Ok(account_vector)
    }
}
