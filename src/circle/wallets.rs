use crate::client::StableflowClient;
use crate::error::StableflowError;
use crate::http::parse_json_or_error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletRequest {
    pub idempotency_key: String,
    pub blockchains: Vec<String>,
    pub count: u32,
    pub wallet_set_id: String,
    pub entity_secret_ciphertext: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateWalletResponse {
    pub data: serde_json::Value,
}

pub async fn create_wallet(
    client: &StableflowClient,
    req: CreateWalletRequest,
) -> Result<CreateWalletResponse, StableflowError> {
    let url = format!("{}/v1/w3s/developer/wallets", client.config.base_url());

    let response = client.http.post(url).json(&req).send().await?;
    parse_json_or_error(response).await
}
