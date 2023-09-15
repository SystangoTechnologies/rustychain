use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::wallet::WalletQueryParams;

#[async_trait]
pub trait WalletService: Sync + Send {
    async fn create_or_update(&self, wallet: Wallet) -> Result<Wallet, CommonError>;
    async fn list(&self, params: WalletQueryParams) -> Result<ResultPaging<Wallet>, CommonError>;
    async fn get(&self, wallet_address: &str, token_address: &str) -> Result<Wallet, CommonError>;
    // async fn delete(&self, block_id: i32) -> Result<(), CommonError>;
}
