use stableflow_rs::circle::ciphertext::generate_entity_secret_ciphertext_for_request;
use stableflow_rs::circle::transactions::CreateTransferRequest;
use stableflow_rs::idempotency::new_idempotency_key;
use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

const ARC_TESTNET_USDC_TOKEN_ADDRESS: &str = "0x3600000000000000000000000000000000000000";
const ARC_TESTNET_BLOCKCHAIN: &str = "ARC-TESTNET";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key =
        std::env::var("CIRCLE_API_KEY").map_err(|_| "missing CIRCLE_API_KEY in .env")?;
    let source_wallet_id =
        std::env::var("CIRCLE_WALLET_ID").map_err(|_| "missing CIRCLE_WALLET_ID in .env")?;
    let destination_address = std::env::var("CIRCLE_DESTINATION_ADDRESS")
        .map_err(|_| "missing CIRCLE_DESTINATION_ADDRESS in .env")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    let entity_secret_ciphertext =
        generate_entity_secret_ciphertext_for_request(&client).await?;

    println!("Sending 1 USDC from:");
    println!("Source wallet ID: {}", source_wallet_id);
    println!("Destination address: {}", destination_address);
    println!("Blockchain: {}", ARC_TESTNET_BLOCKCHAIN);

    let request = CreateTransferRequest {
        idempotency_key: new_idempotency_key(),
        wallet_id: source_wallet_id,
        destination_address,
        amounts: vec!["1".to_string()],
        fee_level: Some("MEDIUM".to_string()),
        entity_secret_ciphertext,
        token_id: None,
        token_address: Some(ARC_TESTNET_USDC_TOKEN_ADDRESS.to_string()),
        blockchain: Some(ARC_TESTNET_BLOCKCHAIN.to_string()),
    };

    let response = client.create_transfer(request).await?;

    println!();
    println!("Transfer submitted:");
    println!("{:#?}", response);

    Ok(())
}
