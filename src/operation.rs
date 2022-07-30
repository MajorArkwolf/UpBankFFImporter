use super::Args;
use crate::config::Config;
use crate::migrator::Migrator;
use crate::{fire_fly, up_bank};
use chrono::{NaiveDate, Utc};
use color_eyre::eyre::{eyre, Result};
use tokio::signal;
use tracing::{debug, error, info};

async fn run_import(
    args: &Args,
    up_bank: &up_bank::UpBank,
    fire_fly: &fire_fly::FireFly,
    config: &Config,
    sleep_duration: &std::time::Duration,
) -> Result<()> {
    import_data(args, up_bank, fire_fly, config).await?;
    debug!("Continues import cycle complete, sleeping until next cycle");
    tokio::time::sleep(*sleep_duration).await;
    Ok(())
}

pub async fn continues_import(
    mut args: Args,
    up_bank: up_bank::UpBank,
    fire_fly: fire_fly::FireFly,
    config: Config,
) -> Result<()> {
    info!("Continues import schedule started.");
    if args.end_date.is_some() || args.start_date.is_some() {
        return Err(eyre!(
            "Start and End date can not be set when using a continues import operation"
        ));
    }

    match args.date_range {
        Some(_) => {}
        None => {
            debug!("No date range specified, using date range of 30");
            args.date_range = Some(30)
        }
    }

    let sleep_duration = chrono::Duration::hours(config.time_between_imports).to_std()?;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Interupt signal recieved, exiting loop");
                break;
            },
            _ = run_import(&args, &up_bank, &fire_fly, &config, &sleep_duration) => {

            }
        }
    }
    Ok(())
}

pub async fn import_data(
    args: &Args,
    up_bank: &up_bank::UpBank,
    fire_fly: &fire_fly::FireFly,
    config: &Config,
) -> Result<()> {
    let account_map = config.get_accounts(up_bank, fire_fly).await?;

    info!("Beginning import...");
    let mut start_date = match &args.start_date {
        Some(date_string) => match NaiveDate::parse_from_str(date_string, "%d-%m-%Y") {
            Ok(date_naive) => {
                info!("Start date set to: {}", date_naive);
                Some(date_naive)
            }
            Err(e) => {
                error!("Failed to parse arg start_date, error: {:?}", e);
                None
            }
        },
        None => None,
    };

    let mut end_date = match &args.end_date {
        Some(date_string) => match NaiveDate::parse_from_str(date_string, "%d-%m-%Y") {
            Ok(date_naive) => {
                info!("End date set to: {}", date_naive);
                Some(date_naive)
            }
            Err(e) => {
                error!("Failed to parse arg end_date, error: {:?}", e);
                None
            }
        },
        None => None,
    };

    if args.date_range.is_some() && start_date.is_some() {
        error!("Date range and start date set, ignoring date range")
    } else if args.date_range.is_some() {
        let end_date_temp = match end_date {
            Some(date) => date,
            None => {
                let today = Utc::now().naive_local().date();
                end_date = Some(today);
                today
            }
        };

        debug!("End date set to: {}", end_date_temp);

        let duration: chrono::Duration = match args.date_range {
            Some(days) => chrono::Duration::days(days),
            None => {
                return Err(eyre!(
                    "argument date range was not set but should have been if it got here."
                ));
            }
        };

        debug!("Date range set to: {}", duration);

        start_date = match end_date_temp.checked_sub_signed(duration) {
            Some(date) => {
                debug!("Start date calculated as: {}", date);
                Some(date)
            }
            None => {
                return Err(eyre!("Failed to determine a valid start date from the date range provided, End Date: {}, Date Range: {}", end_date_temp, duration));
            }
        }
    }

    let mut migrator = Migrator::create(up_bank.clone(), fire_fly.clone(), account_map);
    info!("Beginning migration of data");
    migrator.migrate_transactions(start_date, end_date).await?;

    Ok(())
}

pub fn print_out_up_bank_account_info(up_bank: up_bank::UpBank) -> Result<()> {
    for account in up_bank.accounts {
        info!(
            "Account name: {}, Account Unique ID: {}, Amount: {}",
            account.attributes.display_name, account.id, account.attributes.balance.value
        );
    }
    Ok(())
}
