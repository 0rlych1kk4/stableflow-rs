use stableflow_rs::circle::dev_controlled::DevControlled;
use stableflow_rs::{Environment, StableflowClient, StableflowConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename_override(".env").ok();

    let api_key =
        std::env::var("CIRCLE_API_KEY").map_err(|_| "missing CIRCLE_API_KEY in .env")?;

    let config = StableflowConfig::new(api_key, Environment::Sandbox);
    let client = StableflowClient::new(config)?;

    let dev = DevControlled::new(&client);
    dev.validate_runtime_context().await?;

    println!("Circle native wallet flow groundwork is ready.");
    println!("Next milestone: implement native wallet creation in Rust.");

    Ok(())
}
