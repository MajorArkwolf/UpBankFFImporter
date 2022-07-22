use crate::{fire_fly, up_bank};
use color_eyre::eyre::Result;

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

pub fn convert_up_bank_transaction_to_fire_fly(
    up_bank_transaction: &up_bank::transactions::Transaction,
) -> fire_fly::transaction::TransactionPayload {
    let mut fire_fly_transaction = fire_fly::transaction::TransactionPayload::default();

    fire_fly_transaction.external_id = Some(up_bank_transaction.id.clone());
    fire_fly_transaction.external_url = match &up_bank_transaction.links {
        Some(links) => links.links_self.as_ref().map(|url| url.clone()),
        None => None,
    };
    fire_fly_transaction.amount = up_bank_transaction.attributes.amount.value.clone();
    fire_fly_transaction.currency_code =
        Some(up_bank_transaction.attributes.amount.currency_code.clone());
    fire_fly_transaction.date = up_bank_transaction.attributes.created_at.clone();
    fire_fly_transaction.description = up_bank_transaction.attributes.description.clone();

    if up_bank_transaction.attributes.amount.value_in_base_units < 0 {
        // If value is less then 0, then the transaction is the source

        // fire_fly_transaction.source_id = up_bank_transaction.relationships.account.data.type.id;
        match &up_bank_transaction.relationships.transfer_account.data {
            Some(account) => todo!("validate if the account is our own or not"),
            None => {}
        }
    } else {
        // else the transaction is the destination.

        // Convert this value into a fire fly account id
        // fire_fly_transaction.destination_id = up_bank_transaction.relationships.account.data.type.id;
    }

    match &up_bank_transaction.attributes.round_up {
        Some(round_up) => todo!(),
        None => {}
    }

    return fire_fly_transaction;
}
