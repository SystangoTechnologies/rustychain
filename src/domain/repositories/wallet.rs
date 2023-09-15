use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for WalletQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait WalletRepository: Send + Sync {
    // async fn create(&self, new_block: &CreateBlock) -> RepositoryResult<Block>;
    async fn list(&self, params: WalletQueryParams) -> RepositoryResult<ResultPaging<Wallet>>;
    async fn create_or_update(&self, updated_wallet: &Wallet) -> RepositoryResult<Wallet>;
    async fn get(&self, wallet_address: &str, token_address: &str) -> RepositoryResult<Wallet>;
    // async fn delete(&self, block_id: i32) -> RepositoryResult<()>;
}
