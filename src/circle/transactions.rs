use crate::client::StableflowClient;
use crate::error::StableflowError;
use crate::http::parse_json_or_error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransferRequest {
    pub idempotency_key: String,
    pub wallet_id: String,
    pub destination_address: String,
    pub amounts: Vec<String>,
    pub fee_level: Option<String>,
    pub entity_secret_ciphertext: String,
    pub token_id: Option<String>,
    pub token_address: Option<String>,
    pub blockchain: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransferResponse {
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GetTransactionResponse {
    pub data: serde_json::Value,
}

pub async fn create_transfer(
    client: &StableflowClient,
    req: CreateTransferRequest,
) -> Result<CreateTransferResponse, StableflowError> {
    let url = format!(
        "{}/v1/w3s/developer/transactions/transfer",
        client.config().base_url()
    );

    let response = client.http.post(url).json(&req).send().await?;
    parse_json_or_error(response).await
}

pub async fn get_transaction(
    client: &StableflowClient,
    id: &str,
) -> Result<GetTransactionResponse, StableflowError> {
    let url = format!("{}/v1/w3s/transactions/{}", client.config().base_url(), id);

    let response = client.http.get(url).send().await?;
    parse_json_or_error(response).await
}
