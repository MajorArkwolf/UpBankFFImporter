use color_eyre::eyre::Result;
use crate::config::Config;
use crate::{up_bank, fire_fly};
use crate::migrator::Migrator;
use super::Args;
use tracing::{error, info};
use chrono::NaiveDate;

pub async fn import_data(args: Args, up_bank: up_bank::UpBank, fire_fly: fire_fly::FireFly, config: Config) -> Result<()> {
    let account_map = config.get_accounts(&up_bank, &fire_fly).await?;

    info!("Account validation completed, services connected");

    info!("Begining import of transaction data");

    let start_date = match &args.start_date {
        Some(date_string) => match NaiveDate::parse_from_str(date_string, "%d-%m-%Y") {
            Ok(date_naive) => {
                info!("Start date set to: {}", date_naive);
                Some(date_naive)
            },
            Err(e) => {
                error!("Failed to parse arg start_date, error: {:?}", e);
                None
            }
        },
        None => None,
    };

    let end_date = match &args.end_date {
        Some(date_string) => match NaiveDate::parse_from_str(date_string, "%d-%m-%Y") {
            Ok(date_naive) => {
                info!("End date set to: {}", date_naive);
                Some(date_naive)
            },
            Err(e) => {
                error!("Failed to parse arg end_date, error: {:?}", e);
                None
            }
        },
        None => None,
    };

    let mut migrator = Migrator::create(up_bank, fire_fly, account_map);
    info!("Beginning migration of data");
    migrator.migrate_transactions(start_date, end_date).await?;

    Ok(())
}

pub fn print_out_up_bank_account_info(up_bank: up_bank::UpBank) -> Result<()> {
    for account in up_bank.accounts {
        info!("Account name: {}, Account Unique ID: {}, Amount: {}", account.attributes.display_name, account.id, account.attributes.balance.value);
    }
    Ok(())
}