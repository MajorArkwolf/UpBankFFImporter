mod Mapper;
mod fire_fly;
mod up_bank;
use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    up_pan_token: String,
    #[clap(long, value_parser)]
    fire_fly_pan_token: String,
    #[clap(long, value_parser)]
    fire_fly_base_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let mut up_bank = up_bank::UpBank::create(args.up_pan_token)?;

    up_bank.ping().await?;

    up_bank.populate_data().await?;

    let mut fire_fly = fire_fly::FireFly::create(args.fire_fly_pan_token, args.fire_fly_base_url)?;

    let account = fire_fly.get_account("1").await?;
    println!("{:?}", account);

    Ok(())
}
