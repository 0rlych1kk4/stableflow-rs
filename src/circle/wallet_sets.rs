use crate::client::StableflowClient;
use crate::error::StableflowError;
use crate::http::parse_json_or_error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletSetRequest {
    pub idempotency_key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletSetResponse {
    pub data: WalletSetData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletSetData {
    pub wallet_set: WalletSet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletSet {
    pub id: String,
    pub custody_type: Option<String>,
    pub name: Option<String>,
    pub update_date: Option<String>,
    pub create_date: Option<String>,
}

pub async fn create_wallet_set(
    client: &StableflowClient,
    req: CreateWalletSetRequest,
) -> Result<CreateWalletSetResponse, StableflowError> {
    let url = format!("{}/v1/w3s/developer/walletSets", client.config.base_url());

    let response = client.http.post(url).json(&req).send().await?;
    parse_json_or_error(response).await
}
