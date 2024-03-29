use color_eyre::eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tracing::{debug, error, warn};

use crate::up_bank;

pub enum Status {
    NotFound,      // Not found at all
    FoundExact,    // Found key and hash was identical
    FoundNotExact, // Found but the hash didnt match
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum TransactionType {
    Deposit = 0,
    Withdrawal = 1,
    Transfer = 2,
    TransferDuplicate = 3, // An internal transfer has two transactions, in and out but firefly only wants one. This indicates that the recv transfer was intetionally not imported.
    Duplicate = 4,
}

impl TransactionType {
    pub fn string_to_enum(value: &str) -> TransactionType {
        if value == "transfer" {
            Self::Transfer
        } else if value == "withdrawl" {
            Self::Withdrawal
        } else {
            Self::Deposit
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransactionHash {
    pub id: String,
    pub transaction_type: TransactionType,
    pub hash: u64,
}
pub struct TransactionHashData {
    transaction_map: HashMap<String, TransactionHash>,
}

impl TransactionHash {
    fn new(id: String, transaction_type: TransactionType, hash: u64) -> Self {
        Self {
            id,
            transaction_type,
            hash,
        }
    }
}

impl Drop for TransactionHashData {
    fn drop(&mut self) {
        let wtr = csv::Writer::from_path("./config/transaction.csv");
        match wtr {
            Ok(mut wtr) => self.transaction_map.iter().for_each(move |f| {
                match wtr.serialize(TransactionHash::new(
                    f.0.to_string(),
                    f.1.transaction_type,
                    f.1.hash,
                )) {
                    Ok(_) => {}
                    Err(err) => error!("Failed to serialize transaction to csv file: {}", err),
                }
            }),
            Err(err) => error!("Failed to output transaction data to csv file: {}", err),
        }
    }
}

impl TransactionHashData {
    pub fn open() -> Self {
        let mut transaction_vector: Vec<TransactionHash> = vec![];
        match csv::Reader::from_path("./config/transaction.csv") {
            Ok(mut rdr) => {
                for result in rdr.deserialize() {
                    match result {
                        Ok(value) => transaction_vector.push(value),
                        Err(err) => error!("Failed to deserialise csv value: {}", err),
                    }
                }
            }
            Err(err) => error!("Failed to open file, got the following error: {}", err),
        }
        let mut transaction_map = HashMap::new();
        transaction_vector.into_iter().for_each(|f| {
            if let Some(new_val) = transaction_map.insert(f.id.clone(), f) {
                error!("Key already in map, updated value to: {}", new_val.id)
            }
        });
        Self { transaction_map }
    }

    pub fn find_transaction(&mut self, transaction: &up_bank::transactions::Transaction) -> Status {
        let hash = calculate_hash(&transaction);
        match self.transaction_map.get(&transaction.id) {
            Some(hash_val) => {
                if hash_val.hash == hash {
                    Status::FoundExact
                } else {
                    Status::FoundNotExact
                }
            }
            None => Status::NotFound,
        }
    }

    pub fn add_transaction(
        &mut self,
        transaction: &up_bank::transactions::Transaction,
        transaction_type: TransactionType,
    ) {
        let hash = calculate_hash(&transaction);
        if let Some(hash_val) = self.transaction_map.get(&transaction.id) {
            if hash_val.hash == hash {
                debug!("Transaction found with same hash, no update");
                return;
            }
        }
        if let Some(new_val) = self.transaction_map.insert(
            transaction.id.clone(),
            TransactionHash::new(transaction.id.clone(), transaction_type, hash),
        ) {
            warn!(
                "Transaction id({}) was already found, updated hash to: {:?}",
                transaction.id, new_val
            )
        }
    }

    pub fn update_transaction(
        &mut self,
        transaction: &up_bank::transactions::Transaction,
    ) -> Result<()> {
        let hash = calculate_hash(&transaction);

        let mut current_val = self
            .transaction_map
            .get(&transaction.id)
            .ok_or(eyre!("Should have had a value when calling update"))?
            .clone();

        current_val.hash = hash;

        self.transaction_map
            .insert(transaction.id.clone(), current_val);

        Ok(())
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
