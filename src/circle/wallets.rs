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
#[serde(rename_all = "camelCase")]
pub struct CreateWalletResponse {
    pub data: CreateWalletData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletData {
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsResponse {
    pub data: ListWalletsData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWalletsData {
    pub wallets: Vec<Wallet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub blockchain: String,
    pub custody_type: Option<String>,
    pub wallet_set_id: Option<String>,
    pub create_date: Option<String>,
    pub update_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResponse {
    pub data: WalletBalanceData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceData {
    pub token_balances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub amount: String,
    pub token: Option<TokenInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub symbol: Option<String>,
}

pub async fn create_wallet(
    client: &StableflowClient,
    req: CreateWalletRequest,
) -> Result<CreateWalletResponse, StableflowError> {
    let url = format!("{}/v1/w3s/developer/wallets", client.config().base_url());

    let response = client.http.post(url).json(&req).send().await?;
    parse_json_or_error(response).await
}

pub async fn list_wallets(
    client: &StableflowClient,
) -> Result<ListWalletsResponse, StableflowError> {
    let url = format!("{}/v1/w3s/wallets", client.config().base_url());

    let response = client.http.get(url).send().await?;
    parse_json_or_error(response).await
}

pub async fn get_wallet_balances(
    client: &StableflowClient,
    wallet_id: &str,
) -> Result<WalletBalanceResponse, StableflowError> {
    let url = format!(
        "{}/v1/w3s/wallets/{}/balances",
        client.config().base_url(),
        wallet_id
    );

    let response = client.http.get(url).send().await?;
    parse_json_or_error(response).await
}
