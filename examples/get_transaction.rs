use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key =
        std::env::var("CIRCLE_API_KEY").map_err(|_| "missing CIRCLE_API_KEY in .env")?;
    let tx_id =
        std::env::var("CIRCLE_TX_ID").map_err(|_| "missing CIRCLE_TX_ID in .env")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    let tx = client.get_transaction(&tx_id).await?;

    println!("{:#?}", tx);

    Ok(())
}
