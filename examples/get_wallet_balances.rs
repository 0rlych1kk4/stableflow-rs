use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key = std::env::var("CIRCLE_API_KEY")?;
    let wallet_id = std::env::var("CIRCLE_WALLET_ID")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    let response = client.get_wallet_balances(&wallet_id).await?;

    println!("Balances for wallet {}\n", wallet_id);

    for balance in response.data.token_balances {
        let symbol = balance
            .token
            .and_then(|t| t.symbol)
            .unwrap_or("UNKNOWN".to_string());

        println!("{} {}", balance.amount, symbol);
    }

    Ok(())
}
