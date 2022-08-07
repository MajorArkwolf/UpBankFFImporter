use crate::{
    fire_fly::{self, transaction::TransactionPayload},
    up_bank,
};
use color_eyre::eyre::{eyre, Result};

pub enum TransferType {
    Transaction(TransactionPayload),
    TransactionDuplicate,
}

use super::account_map;

pub async fn find_up_bank_transaction_in_fire_fly(
    up_bank_transaction: &up_bank::transactions::Transaction,
    fire_fly: &fire_fly::FireFly,
) -> Result<bool> {
    let transction_data = fire_fly
        .find_transaction_by_external_id(&up_bank_transaction.id)
        .await?;
    Ok(!transction_data.is_empty())
}

pub async fn get_fire_fly_transction_from_up_bank_id(
    up_bank_transaction: &up_bank::transactions::Transaction,
    fire_fly: &fire_fly::FireFly,
) -> Result<Vec<fire_fly::transaction::TransactionData>> {
    fire_fly
        .find_transaction_by_external_id(&up_bank_transaction.id)
        .await
}

pub fn is_account_internal(
    account_id: &str,
    account_map: &[account_map::AccountMap],
) -> Option<String> {
    account_map
        .iter()
        .find(|x| x.up_account_id == account_id)
        .map(|result| result.fire_fly_account_id.clone())
}

pub fn convert_up_bank_transaction_to_fire_fly(
    up_bank_transaction: &up_bank::transactions::Transaction,
    account_map: &[account_map::AccountMap],
) -> Result<TransferType> {
    let mut fire_fly_transaction = fire_fly::transaction::TransactionPayload::default();

    fire_fly_transaction.external_id = Some(up_bank_transaction.id.clone());
    fire_fly_transaction.external_url = match &up_bank_transaction.links {
        Some(links) => links.links_self.as_ref().cloned(),
        None => None,
    };
    fire_fly_transaction.amount = up_bank_transaction
        .attributes
        .amount
        .value
        .clone()
        .replace('-', "");
    fire_fly_transaction.currency_code =
        Some(up_bank_transaction.attributes.amount.currency_code.clone());
    fire_fly_transaction.date = up_bank_transaction.attributes.created_at.clone();
    fire_fly_transaction.description = up_bank_transaction.attributes.description.clone();
    match &up_bank_transaction.attributes.raw_text {
        Some(text) => {
            fire_fly_transaction.description =
                format!("{}, {}", fire_fly_transaction.description, text)
        }
        None => {}
    }

    fire_fly_transaction.order = Some(0); // Unsure what value should be here, however it is required to be populated

    fire_fly_transaction.category_name = up_bank_transaction
        .relationships
        .category
        .data
        .as_ref()
        .map(|f| f.id.clone().replace('-', "_"));

    match &up_bank_transaction.attributes.foreign_amount {
        Some(foriegn_amount) => {
            fire_fly_transaction.foreign_amount = Some(foriegn_amount.value.clone());
            fire_fly_transaction.foreign_currency_code = Some(foriegn_amount.currency_code.clone());
        }
        None => fire_fly_transaction.foreign_amount = Some("0".to_string()),
    }

    if up_bank_transaction.attributes.amount.value_in_base_units < 0 {
        fire_fly_transaction.transaction_type = "withdrawal".to_string();
        // If value is less then 0, then the transaction is the source
        match up_bank_transaction
            .relationships
            .account
            .data
            .as_ref()
            .ok_or("this should have been a value")
        {
            Ok(data) => {
                let account = is_account_internal(&data.id, account_map).ok_or(eyre!(
                    "account should have mapped across from upbank to firefly"
                ))?;
                fire_fly_transaction.source_id = Some(account);
            }
            Err(e) => return Err(eyre!("this should have contained a valid id, error: {}", e)),
        };

        match &up_bank_transaction.relationships.transfer_account.data {
            Some(transfer_account) => {
                match is_account_internal(&transfer_account.id, account_map) {
                    Some(_fire_fly_id) => {
                        return Ok(TransferType::TransactionDuplicate); // To avoid duplicate transfers from showing up we return None
                                                                       //fire_fly_transaction.destination_id = Some(fire_fly_id);
                                                                       // Since this is moving accounts we create a transfer.
                                                                       //fire_fly_transaction.transaction_type = "transfer".to_string();
                    } // If its an account mapped in firefly then its better to link it directly.
                    None => {
                        fire_fly_transaction.destination_name =
                            Some(transfer_account.dat_type.clone())
                    } // Else just link the name of the account instead.
                }
            }
            None => {
                fire_fly_transaction.destination_name =
                    Some(up_bank_transaction.attributes.description.clone())
            }
        }
    } else {
        fire_fly_transaction.transaction_type = "deposit".to_string();

        // else the transaction is the destination.
        match up_bank_transaction
            .relationships
            .account
            .data
            .as_ref()
            .ok_or("this should have been a value")
        {
            Ok(data) => {
                let account = is_account_internal(&data.id, account_map).ok_or_else(|| {
                    eyre!(
                    "account should have mapped across from upbank to firefly, account number: {}",
                    data.id
                )
                })?;
                fire_fly_transaction.destination_id = Some(account);
            }
            Err(e) => return Err(eyre!("this should have contained a valid id, error: {}", e)),
        };

        match &up_bank_transaction.relationships.transfer_account.data {
            Some(transfer_account) => {
                match is_account_internal(&transfer_account.id, account_map) {
                    Some(fire_fly_id) => {
                        fire_fly_transaction.source_id = Some(fire_fly_id);
                        fire_fly_transaction.transaction_type = "transfer".to_string();
                        //return Ok(TransferType::TransactionDuplicate); // To avoid duplicate transfers from showing up we return None
                    }
                    None => {
                        fire_fly_transaction.source_name = Some(transfer_account.dat_type.clone())
                    } // Else just link the name of the account instead.
                }
            }
            None => {
                fire_fly_transaction.source_name =
                    Some(up_bank_transaction.attributes.description.clone())
            }
        }
    }

    Ok(TransferType::Transaction(fire_fly_transaction))
}
