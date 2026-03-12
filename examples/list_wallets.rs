use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key =
        std::env::var("CIRCLE_API_KEY").map_err(|_| "missing CIRCLE_API_KEY in .env")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    let response = client.list_wallets().await?;

    println!("Found {} wallet(s)", response.data.wallets.len());
    for wallet in response.data.wallets {
        println!("----------------------------------------");
        println!("ID: {}", wallet.id);
        println!("Address: {}", wallet.address);
        println!("Blockchain: {}", wallet.blockchain);
        if let Some(wallet_set_id) = wallet.wallet_set_id {
            println!("Wallet Set ID: {}", wallet_set_id);
        }
        if let Some(custody_type) = wallet.custody_type {
            println!("Custody Type: {}", custody_type);
        }
    }

    Ok(())
}
