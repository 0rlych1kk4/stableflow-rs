use stableflow_rs::{Environment, StableflowClient, StableflowConfig};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key =
        std::env::var("CIRCLE_API_KEY").map_err(|_| "missing CIRCLE_API_KEY in .env")?;

    let tx_id =
        std::env::var("CIRCLE_TX_ID").map_err(|_| "missing CIRCLE_TX_ID in .env")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    println!("Waiting for transaction: {}", tx_id);

    loop {
        let response = client.get_transaction(&tx_id).await?;

        let tx = &response.data["transaction"];
        let state = tx["state"].as_str().unwrap_or("UNKNOWN");

        println!("Current state: {}", state);

        if state == "COMPLETE" {
            println!();
            println!("Transaction completed.");
            println!("Tx Hash: {}", tx["txHash"]);
            break;
        }

        if state == "FAILED" {
            println!("Transaction failed.");
            break;
        }

        sleep(Duration::from_secs(3)).await;
    }

    Ok(())
}
