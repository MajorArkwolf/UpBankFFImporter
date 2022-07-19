use crate::{up_bank, fire_fly};
use color_eyre::eyre::{Result};


pub async fn find_up_bank_transaction_in_fire_fly(up_bank_transaction: &up_bank::transactions::Transaction, fire_fly: &fire_fly::FireFly) -> Result<bool> {
    let transction_data = fire_fly.find_transaction_by_external_id(&up_bank_transaction.id).await?;
    Ok(!transction_data.is_empty())
}

pub async fn get_fire_fly_transction_from_up_bank_id(up_bank_transaction: &up_bank::transactions::Transaction, fire_fly: &fire_fly::FireFly) -> Result<Vec<fire_fly::transaction::TransactionData>> {
    fire_fly.find_transaction_by_external_id(&up_bank_transaction.id).await
}

pub fn convert_up_bank_transaction_to_fire_fly(up_bank_transaction: &up_bank::transactions::Transaction) -> fire_fly::transaction::TransactionPayload {
    let mut fire_fly_transaction = fire_fly::transaction::TransactionPayload::default();

    fire_fly_transaction.external_id = Some(up_bank_transaction.id.clone());
    //fire_fly_transaction.external_url = Some()
    fire_fly_transaction.date = up_bank_transaction.attributes.created_at.clone();
    fire_fly_transaction.description = up_bank_transaction.attributes.description.clone();

    return fire_fly_transaction;
}

pub async fn insert_up_bank_transaction_into_fire_fly(up_bank_transaction: &up_bank::transactions::Transaction, fire_fly: &fire_fly::FireFly) -> Result<()> {

    Ok(())
}