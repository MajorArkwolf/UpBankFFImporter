use crate::{fire_fly, up_bank};
use color_eyre::eyre::{Result};
use self::account_map::AccountMap;

pub mod account_map;
pub mod transaction_map;

pub struct Migrator {
    up_bank_api: up_bank::UpBank,
    fire_fly_api: fire_fly::FireFly,
    account_map: AccountMap,
}

impl Migrator {
    pub fn create(
        up_bank_api: up_bank::UpBank,
        fire_fly_api: fire_fly::FireFly,
        account_map: AccountMap,
    ) -> Self {
        Self {up_bank_api, fire_fly_api, account_map}
    }

    pub fn migrate_transaction(start_date: Option<chrono::naive::NaiveDate>, end_date: Option<chrono::naive::NaiveDate>) -> Result<()> {
        
        Ok(())
    }
}
