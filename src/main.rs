mod up_bank;
use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    up_pan_token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let mut up_bank = up_bank::UpBank::create(args.up_pan_token)?;

    up_bank.ping().await?;

    up_bank.populate_data().await?;

    Ok(())
}
