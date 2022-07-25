use crate::{up_bank, fire_fly};

use self::account_map::AccountMap;
use color_eyre::eyre::Result;
use tracing::{error, info, debug};

pub mod account_map;
pub mod transaction_map;

pub struct Migrator {
    up_bank_api: up_bank::UpBank,
    fire_fly_api: fire_fly::FireFly,
    account_map: Vec<AccountMap>,
}

impl Migrator {
    pub fn create(
        up_bank_api: up_bank::UpBank,
        fire_fly_api: fire_fly::FireFly,
        account_map: Vec<AccountMap>,
    ) -> Self {
        Self {
            up_bank_api,
            fire_fly_api,
            account_map,
        }
    }

    pub async fn migrate_transactions(
        &self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
    ) -> Result<()> {
        let up_bank_transaction = self
            .up_bank_api
            .get_all_transactions(start_date, end_date)
            .await?;

        let account_map = &self.account_map;
        let fire_fly = &self.fire_fly_api;

        let up_bank_transaction: Vec<up_bank::transactions::Transaction> = up_bank_transaction
        .into_iter()
        .filter(|e| account_map.into_iter().any(|f| f.up_account_id == e.relationships.account.data.as_ref().unwrap().id))
        .collect();



        info!("Importing {} transactions", up_bank_transaction.len());

        for transaction in up_bank_transaction {
            let was_found = transaction_map::find_up_bank_transaction_in_fire_fly(&transaction, fire_fly).await?;
            if !was_found {
                debug!("Importing up bank transaction: {}", transaction.id);
                match self.migrate_transaction(&transaction).await {
                    Ok(_) => continue,
                    Err(e) => error!(
                        "Transaction({}) failed to import, error: {:?}",
                        transaction.id, e
                    ),
                }
            } else {
                debug!("Transaction {} was already found in fire fly", transaction.id)
            }
        }

        info!("Import complete");

        Ok(())
    }

    pub async fn migrate_transaction(
        &self,
        up_bank_transaction: &up_bank::transactions::Transaction,
    ) -> Result<()> {
        let fire_fly_payload =
            transaction_map::convert_up_bank_transaction_to_fire_fly(up_bank_transaction, &self.account_map)?;
        self.fire_fly_api
            .submit_new_transaction(&fire_fly_payload)
            .await?;
        Ok(())
    }
}
