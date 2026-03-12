use crate::client::StableflowClient;
use crate::error::StableflowError;

pub struct DevControlled<'a> {
    client: &'a StableflowClient,
}

impl<'a> DevControlled<'a> {
    pub fn new(client: &'a StableflowClient) -> Self {
        Self { client }
    }

    pub async fn validate_runtime_context(&self) -> Result<(), StableflowError> {
        let secret = self.client.entity_secret()?;

        if secret.len() != 64 {
            return Err(StableflowError::InvalidConfig(
                "CIRCLE_ENTITY_SECRET must be a 32-byte hex string (64 hex chars)".into(),
            ));
        }

        println!("Dev-controlled runtime context validated.");
        println!("Entity secret length: {}", secret.len());

        Ok(())
    }
}
