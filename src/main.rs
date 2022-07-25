pub mod config;
pub mod fire_fly;
pub mod migrator;
pub mod up_bank;
use chrono::NaiveDate;
use clap::Parser;
use color_eyre::eyre::Result;
use migrator::Migrator;
use tracing::{error, info};

use config::Config;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    start_date: Option<String>,
    #[clap(short, long, value_parser)]
    end_date: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("Starting services");
    let config = Config::load("./settings.yaml")?;
    info!("Loaded config file");
    let args = Args::parse();
    info!("Parsed arguments and updated config");

    let mut up_bank = up_bank::UpBank::create(config.up_pan_token.clone())?;
    let fire_fly = fire_fly::FireFly::create(
        config.fire_fly_pan_token.clone(),
        config.fire_fly_base_url.clone(),
    )?;
    info!("FireFly and UpBank api initilised, but not connected yet");

    let start_date = match &args.start_date {
        Some(date_string) => match NaiveDate::parse_from_str(date_string, "%d-%m-%Y") {
            Ok(date_naive) => Some(date_naive),
            Err(e) => {
                error!("Failed to parse arg start_date, error: {:?}", e);
                None
            }
        },
        None => None,
    };

    let end_date = match &args.end_date {
        Some(date_string) => match NaiveDate::parse_from_str(date_string, "%d-%m-%Y") {
            Ok(date_naive) => Some(date_naive),
            Err(e) => {
                error!("Failed to parse arg end_date, error: {:?}", e);
                None
            }
        },
        None => None,
    };

    up_bank.populate_data().await?;

    let account_map = config.get_accounts()?;
    for account in &account_map {
        account.validate(&up_bank, &fire_fly).await?
    }
    info!("Account validation completed, services connected");

    let migrator = Migrator::create(up_bank, fire_fly, account_map);
    migrator.migrate_transactions(start_date, end_date).await?;

    Ok(())
}
