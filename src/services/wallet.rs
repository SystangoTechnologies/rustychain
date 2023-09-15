// use chrono::prelude::*;
use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::wallet::WalletQueryParams;
use crate::domain::repositories::wallet::WalletRepository;
use crate::domain::services::wallet::WalletService;

#[derive(Clone)]
pub struct WalletServiceImpl {
    pub repository: Arc<dyn WalletRepository>,
}

impl WalletServiceImpl {
    pub fn new(repository: Arc<dyn WalletRepository>) -> Self {
        WalletServiceImpl { repository }
    }
}

#[async_trait]
impl WalletService for WalletServiceImpl {
    async fn create_or_update(&self, wallet: Wallet) -> Result<Wallet, CommonError> {
        let created_block = self.repository.create_or_update(&wallet).await.map_err(|e| -> CommonError { e.into() })?;

        Ok(created_block)
    }

    async fn get(&self, wallet_address: &str, token_address: &str) -> Result<Wallet, CommonError> {
        self.repository.get(wallet_address, token_address).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: WalletQueryParams) -> Result<ResultPaging<Wallet>, CommonError> {
        self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
    }
}
