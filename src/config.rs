use super::migrator::account_map::AccountMap;
use color_eyre::eyre::{Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::vec;

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

    pub fn override_with_args(&mut self, args: super::Args) {
        if args.up_pan_token.is_some() {
            self.up_pan_token = args.up_pan_token.unwrap();
        }

        if args.fire_fly_pan_token.is_some() {
            self.up_pan_token = args.fire_fly_pan_token.unwrap();
        }

        if args.fire_fly_base_url.is_some() {
            self.up_pan_token = args.fire_fly_base_url.unwrap();
        }
    }

    pub fn get_accounts(&self) -> Result<Vec<AccountMap>> {
        let mut account_vector: Vec<AccountMap> = vec![];
        for accounts in &self.account_mapping {
            let account_vec: Vec<String> =
                accounts.clone().split(':').map(|s| s.to_string()).collect();

            let new_account = AccountMap::create(account_vec[0].clone(), account_vec[1].clone());
            account_vector.push(new_account);
        }

        Ok(account_vector)
    }
}
