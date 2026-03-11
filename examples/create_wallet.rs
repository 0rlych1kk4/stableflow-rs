use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

fn reject_placeholder(name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lowered = value.trim().to_ascii_lowercase();

    if lowered.is_empty()
        || lowered.contains("your_")
        || lowered.contains("replace_me")
        || lowered.contains("placeholder")
    {
        return Err(format!(
            "{name} is still a placeholder. Update your .env with the real value."
        )
        .into());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key =
        std::env::var("CIRCLE_API_KEY").map_err(|_| "missing CIRCLE_API_KEY in .env")?;
    let wallet_set_id = std::env::var("CIRCLE_WALLET_SET_ID")
        .map_err(|_| "missing CIRCLE_WALLET_SET_ID in .env")?;

    reject_placeholder("CIRCLE_API_KEY", &api_key)?;
    reject_placeholder("CIRCLE_WALLET_SET_ID", &wallet_set_id)?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    let entity_secret = client.entity_secret()?;

    println!("Using base URL: {}", client.config().base_url());
    println!("Wallet Set ID: {}", wallet_set_id);
    println!("Entity Secret loaded: {} chars", entity_secret.len());
    println!();
    println!("Circle setup is initialized correctly.");
    println!("Next step: implement native sensitive request handling in Rust.");

    Ok(())
}
