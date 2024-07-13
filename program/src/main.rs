use dotenv::dotenv;
use eyre::Result;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // initialize tracing
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Generating proofs...");

    Ok(())
}
