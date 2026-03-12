use crate::client::StableflowClient;
use crate::error::StableflowError;

pub struct DevControlled<'a> {
    client: &'a StableflowClient,
}

impl<'a> DevControlled<'a> {
    pub fn new(client: &'a StableflowClient) -> Self {
        Self { client }
    }

    pub async fn create_wallet_placeholder(&self) -> Result<(), StableflowError> {
        let secret = self.client.entity_secret()?;

        println!("Dev-controlled flow initialized.");
        println!("Entity secret length: {}", secret.len());

        // TODO: Implement native Circle wallet creation

        Ok(())
    }
}
