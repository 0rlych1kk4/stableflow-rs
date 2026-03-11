use stableflow_rs::circle::wallet_sets::CreateWalletSetRequest;
use stableflow_rs::idempotency::new_idempotency_key;
use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("CIRCLE_API_KEY")
        .map_err(|_| "missing CIRCLE_API_KEY in environment or .env")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    println!("Using base URL: {}", config.base_url());

    let client = StableflowClient::new(config)?;

    let response = client
        .create_wallet_set(CreateWalletSetRequest {
            idempotency_key: new_idempotency_key(),
            name: "stableflow-dev".to_string(),
        })
        .await?;

    println!("Wallet Set created successfully:");
    println!("{:#?}", response);
    println!("CIRCLE_WALLET_SET_ID={}", response.data.wallet_set.id);

    Ok(())
}
