mod config;
mod fire_fly;
mod mapper;
mod up_bank;
use clap::Parser;
use color_eyre::eyre::Result;
use tracing::{debug, info};

use config::Config;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    up_pan_token: Option<String>,
    #[clap(long, value_parser)]
    fire_fly_pan_token: Option<String>,
    #[clap(long, value_parser)]
    fire_fly_base_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("Starting services");
    let mut config = Config::load("./settings.yaml")?;
    info!("Loaded config file");
    let args = Args::parse();

    config.override_with_args(args);
    info!("Parsed arguments and updated config");

    let mut up_bank = up_bank::UpBank::create(config.up_pan_token.clone())?;
    let mut fire_fly = fire_fly::FireFly::create(
        config.fire_fly_pan_token.clone(),
        config.fire_fly_base_url.clone(),
    )?;
    info!("FireFly and UpBank api initilised, but not connected yet");

    let account_map = config.get_accounts()?;
    for account in account_map {
        account.validate(&up_bank, &fire_fly).await?
    }
    info!("Account validation completed, services connected");

    up_bank.ping().await?;

    up_bank.populate_data().await?;

    //println!("{:?}", up_bank.accounts);

    let account = fire_fly.get_account("1").await?;
    println!("{:?}", account);

    Ok(())
}
