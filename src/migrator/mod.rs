use std::collections::HashSet;

use crate::{
    fire_fly, migrator::transaction_map::get_fire_fly_transction_from_up_bank_id, up_bank,
};

use self::{
    account_map::AccountMap,
    transaction_tracker::{TransactionHashData, TransactionType},
};
use color_eyre::eyre::{eyre, Result};
use tracing::{debug, error, info};

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
            .filter(|e| {
                self.account_map
                    .iter()
                    .any(|f| f.up_account_id == e.relationships.account.data.as_ref().unwrap().id)
            })
            .collect();

        info!("Processing {} transactions", up_bank_transaction.len());
        let mut not_found_counter = 0;
        let mut needs_update_counter = 0;
        let mut already_imported_counter = 0;

        let tag: String = "UBFF3Import".to_string();

        for transaction in up_bank_transaction {
            match self.transaction_tracker.find_transaction(&transaction) {
                transaction_tracker::Status::NotFound => {
                    if self.new_transaction(&transaction, &tag).await? {
                        not_found_counter += 1;
                    } else {
                        // Since we do not already have a hash we wont know if it needs to be updated.
                        already_imported_counter += 1;
                        self.transaction_tracker
                            .add_transaction(&transaction, TransactionType::Duplicate);
                    }
                }
                transaction_tracker::Status::FoundExact => {
                    debug!(
                        "Transaction({}) found in TransactionMap with no update required, skipping",
                        transaction.id
                    );
                    already_imported_counter += 1;
                    self.transaction_tracker
                        .add_transaction(&transaction, TransactionType::Duplicate);
                }
                transaction_tracker::Status::FoundNotExact => {
                    self.update_transaction(&transaction).await?;
                    needs_update_counter += 1;
                }
            };
        }

        info!("Import complete, {} new transactions, {} updated transactions and {} were already imported and identical", not_found_counter, needs_update_counter, already_imported_counter);

        Ok(())
    }

    pub async fn update_transaction(
        &mut self,
        transaction: &up_bank::transactions::Transaction,
    ) -> Result<()> {
        let fire_fly_transactions =
            get_fire_fly_transction_from_up_bank_id(transaction, &self.fire_fly_api).await?;

        // Error out if multiple transactions are found as cant determine which one should be updated.
        if fire_fly_transactions.len() != 1 {
            return Err(eyre!("Only a single transaction matching a external id should have been in fire_fly, however {} were returned. External ID: {}", fire_fly_transactions.len(), transaction.id));
        }

        // Remove the single transaction out of the array
        let fire_fly_transaction = fire_fly_transactions
            .into_iter()
            .next()
            .ok_or(eyre!("A transaction should have existed in the array"))?;

        let mut fire_fly_transaction = fire_fly_transaction
            .attributes
            .transactions
            .into_iter()
            .next()
            .ok_or_else(|| eyre!("A transaction should have existed here"))?;

        // Grab the latest category
        fire_fly_transaction.category_name = transaction
            .relationships
            .category
            .data
            .as_ref()
            .map(|f| f.id.clone().replace('-', "_"));

        // Collect all the tags in up bank
        let transaction_tags: Vec<String> = transaction
            .relationships
            .tags
            .data
            .iter()
            .map(|f| f.dat_type.clone())
            .collect();

        // Merge tags into one
        fire_fly_transaction.tags.extend(transaction_tags);

        // Remove duplicates
        dedup(&mut fire_fly_transaction.tags);

        self.fire_fly_api
            .update_transaction(fire_fly_transaction)
            .await?;
        self.transaction_tracker.update_transaction(transaction)?;
        Ok(())
    }

    pub async fn new_transaction(
        &mut self,
        transaction: &up_bank::transactions::Transaction,
        tag: &str,
    ) -> Result<bool> {
        let was_found =
            transaction_map::find_up_bank_transaction_in_fire_fly(transaction, &self.fire_fly_api)
                .await?;
        if !was_found {
            debug!("Importing up bank transaction: {}", transaction.id);
            match self
                .migrate_transaction(transaction, &Some(tag.to_string()))
                .await
            {
                Ok(resp) => {
                    self.transaction_tracker.add_transaction(transaction, resp);
                }
                Err(e) => error!(
                    "Transaction({}) failed to import, error: {:?}",
                    transaction.id, e
                ),
            }
        } else {
            debug!(
                "Transaction {} was already found in fire fly",
                transaction.id
            )
        }

        Ok(!was_found)
    }

    pub async fn migrate_transaction(
        &self,
        up_bank_transaction: &up_bank::transactions::Transaction,
        import_tag: &Option<String>,
    ) -> Result<TransactionType> {
        match transaction_map::convert_up_bank_transaction_to_fire_fly(
            up_bank_transaction,
            &self.account_map,
        )? {
            transaction_map::TransferType::Transaction(mut fire_fly_payload) => {
                match import_tag {
                    Some(tag) => {
                        fire_fly_payload.tags.push(tag.to_string());
                    }
                    None => {}
                };
                self.fire_fly_api
                    .submit_new_transaction(&fire_fly_payload)
                    .await?;
                Ok(TransactionType::string_to_enum(
                    &fire_fly_payload.transaction_type,
                ))
            }
            transaction_map::TransferType::TransactionDuplicate => {
                Ok(TransactionType::TransferDuplicate)
            }
        }
    }
}

fn dedup(v: &mut Vec<String>) {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(e.clone()));
}
