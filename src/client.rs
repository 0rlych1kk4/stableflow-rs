use crate::circle::{transactions, wallet_sets, wallets};
use crate::config::StableflowConfig;
use crate::error::StableflowError;
use crate::http::build_http_client;

#[derive(Clone)]
pub struct StableflowClient {
    pub(crate) http: reqwest::Client,
    pub(crate) config: StableflowConfig,
}

impl StableflowClient {
    pub fn new(config: StableflowConfig) -> Result<Self, StableflowError> {
        let http = build_http_client(&config)?;
        Ok(Self { http, config })
    }

    pub async fn create_wallet_set(
        &self,
        req: wallet_sets::CreateWalletSetRequest,
    ) -> Result<wallet_sets::CreateWalletSetResponse, StableflowError> {
        wallet_sets::create_wallet_set(self, req).await
    }

    pub async fn create_wallet(
        &self,
        req: wallets::CreateWalletRequest,
    ) -> Result<wallets::CreateWalletResponse, StableflowError> {
        wallets::create_wallet(self, req).await
    }

    pub async fn create_transfer(
        &self,
        req: transactions::CreateTransferRequest,
    ) -> Result<transactions::CreateTransferResponse, StableflowError> {
        transactions::create_transfer(self, req).await
    }

    pub async fn get_transaction(
        &self,
        id: &str,
    ) -> Result<transactions::GetTransactionResponse, StableflowError> {
        transactions::get_transaction(self, id).await
    }
}
