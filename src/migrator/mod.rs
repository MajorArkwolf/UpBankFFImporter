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

    pub async fn migrate_transactions(&self, start_date: Option<chrono::naive::NaiveDate>, end_date: Option<chrono::naive::NaiveDate>) -> Result<()> {

        Ok(())
    }

    pub async fn migrate_transaction(&self, up_bank_transaction: &up_bank::transactions::Transaction) -> Result<()> {
        let fire_fly_payload = transaction_map::convert_up_bank_transaction_to_fire_fly(up_bank_transaction);
        self.fire_fly_api.submit_new_transaction(&fire_fly_payload).await?;
        Ok(())
    }
}
