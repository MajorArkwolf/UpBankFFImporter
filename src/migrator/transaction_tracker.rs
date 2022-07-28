use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use color_eyre::eyre::Result;

use serde::{Deserialize, Serialize};
use tracing::{error, warn, debug};

use crate::up_bank;

pub enum Status {
    NotFound, // Not found at all
    FoundExact, // Found key and hash was identical
    FoundNotExact, // Found but the hash didnt match
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransactionHash {
    pub id: String,
    pub hash: u64,
}
pub struct TransactionHashData {
    transaction_map: HashMap<String, u64>,
}

impl TransactionHash {
    fn new(id: String, hash: u64) -> Self {
        Self{id, hash}
    }
}

impl Drop for TransactionHashData {
    fn drop(&mut self) {
        let wtr = csv::Writer::from_path("./config/transaction.csv");
        match wtr {
            Ok(mut wtr) => {
                self.transaction_map.iter().for_each(move |f| match wtr.serialize(TransactionHash::new(f.0.to_string(), *f.1)){
                    Ok(_) => {},
                    Err(err) => error!("Failed to serialize transaction to csv file: {}", err),
                })
            },
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
                        Ok(value) => {
                            transaction_vector.push(value)
                        },
                        Err(err) => error!("Failed to deserialise csv value: {}", err),
                    }
                }
            },
            Err(err) => error!("Failed to open file, got the following error: {}", err),
        }
        let mut transaction_map = HashMap::new();
        transaction_vector.into_iter().for_each(|f| match transaction_map.insert(f.id, f.hash) {
            Some(new_val) => {error!("Key already in map, updated value to: {}", new_val)},
            None => {},
        });
        Self {transaction_map}
    }

    pub fn find_transaction(&mut self, transaction: &up_bank::transactions::Transaction) -> Status
    {
        let hash = calculate_hash(&transaction);
        match self.transaction_map.get(&transaction.id) {
            Some(hash_val) => {if *hash_val == hash {
                return Status::FoundExact;
            } else {
                return Status::FoundNotExact;
            }
        },
            None => return Status::NotFound,
        }
    }

    pub fn add_transaction(&mut self, transaction: &up_bank::transactions::Transaction) {
        let hash = calculate_hash(&transaction);
        match self.transaction_map.get(&transaction.id) {
            Some(hash_val) => if *hash_val == hash {
                debug!("Transaction found with same hash, no update");
                return;
            },
            None => {},
        }
        match self.transaction_map.insert(transaction.id.clone(), hash) {
            Some(new_val) => warn!("Transaction id({}) was already found, updated hash to: {}", transaction.id, new_val),
            None => {},
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}