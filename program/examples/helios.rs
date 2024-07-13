use std::{path::PathBuf, str::FromStr, env};

use dotenv::dotenv;
use ethers::{types::Address, utils};
use eyre::Result;
use helios::{client::ClientBuilder, config::networks::Network, prelude::*, types::BlockTag};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // initialize tracing
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let untrusted_rpc_url = env::var("UNTRUSTED_RPC_URL")?;
    info!("Untrusted RPC URL: {}", untrusted_rpc_url);

    // initialize helios client
    let mut client: helios::client::Client<FileDB> = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc("https://www.lightclientdata.org")
        .execution_rpc(&untrusted_rpc_url)
        .load_external_fallback()
        .data_dir(PathBuf::from("/tmp/helios"))
        .build()?;

    client.start().await?;
    client.wait_synced().await;

    let head_block_num = client.get_block_number().await?;
    let addr = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa")?;
    let block = BlockTag::Latest;
    let balance = client.get_balance(&addr, block).await?;

    info!("synced up to block: {}", head_block_num);
    info!("balance of deposit contract: {}", utils::format_ether(balance));

    Ok(())
}
