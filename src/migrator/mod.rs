use crate::{up_bank, fire_fly};

use self::{account_map::AccountMap, transaction_tracker::TransactionHashData};
use color_eyre::eyre::Result;
use tracing::{error, info, debug};

pub mod account_map;
pub mod transaction_map;
pub mod transaction_tracker;

pub struct Migrator {
    up_bank_api: up_bank::UpBank,
    fire_fly_api: fire_fly::FireFly,
    account_map: Vec<AccountMap>,
    transaction_tracker: TransactionHashData,
}

impl Migrator {
    pub fn create(
        up_bank_api: up_bank::UpBank,
        fire_fly_api: fire_fly::FireFly,
        account_map: Vec<AccountMap>,
    ) -> Self {
        let transaction_tracker = TransactionHashData::open();
        Self {
            up_bank_api,
            fire_fly_api,
            account_map,
            transaction_tracker,
        }
    }

    pub async fn migrate_transactions(
        &mut self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
    ) -> Result<()> {
        let up_bank_transaction = self
            .up_bank_api
            .get_all_transactions(start_date, end_date)
            .await?;

        let up_bank_transaction: Vec<up_bank::transactions::Transaction> = up_bank_transaction
        .into_iter()
        .filter(|e| self.account_map.iter().any(|f| f.up_account_id == e.relationships.account.data.as_ref().unwrap().id))
        .collect();

        info!("Importing {} transactions", up_bank_transaction.len());

        let tag = format!("UBFF3Import-{}", chrono::offset::Local::now());

        for transaction in up_bank_transaction {
            match self.transaction_tracker.find_transaction(&transaction) {
                transaction_tracker::Status::NotFound => { self.new_transaction(&transaction, &tag).await?} ,
                transaction_tracker::Status::FoundExact => { debug!("Transaction({}) found in TransactionMap with no update required, skipping", transaction.id) },
                transaction_tracker::Status::FoundNotExact => self.update_transaction(&transaction, &tag).await,
            };
        }

        info!("Import complete");

        Ok(())
    }

    pub async fn update_transaction(&mut self, transaction: &up_bank::transactions::Transaction, tag: &str) {
        debug!("coming soon tm");
    }

    pub async fn new_transaction(&mut self, transaction: &up_bank::transactions::Transaction, tag: &str) -> Result<()> {
        let was_found = transaction_map::find_up_bank_transaction_in_fire_fly(&transaction, &self.fire_fly_api).await?;
        if !was_found {
            debug!("Importing up bank transaction: {}", transaction.id);
            match self.migrate_transaction(&transaction, &Some(tag.to_string())).await {
                Ok(_) => {},
                Err(e) => error!(
                    "Transaction({}) failed to import, error: {:?}",
                    transaction.id, e
                ),
            }
        } else {
            debug!("Transaction {} was already found in fire fly", transaction.id)
        }
        self.transaction_tracker.add_transaction(&transaction);
        Ok(())
    }

    pub async fn migrate_transaction(
        &self,
        up_bank_transaction: &up_bank::transactions::Transaction,
        import_tag: &Option<String>
    ) -> Result<()> {
        let mut fire_fly_payload =
            transaction_map::convert_up_bank_transaction_to_fire_fly(up_bank_transaction, &self.account_map)?;
        match import_tag {
            Some(tag) => fire_fly_payload.tags.push(tag.to_string()),
            None => {},
        }
        self.fire_fly_api
            .submit_new_transaction(&fire_fly_payload)
            .await?;
        
        Ok(())
    }
}
