use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::fungible_token::FungibleToken;
use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::fungible_token::FungibleTokenQueryParams;
use crate::domain::repositories::repository::ResultPaging;

#[async_trait]
pub trait FungibleTokenService: Sync + Send {
    async fn create(&self, create_fungible_token: FungibleToken) -> Result<FungibleToken, CommonError>;
    async fn list(&self, params: FungibleTokenQueryParams) -> Result<ResultPaging<FungibleToken>, CommonError>;
    async fn get(&self, token_address: &str) -> Result<FungibleToken, CommonError>;
    async fn mint(&self, token_address: &str, requester_address: &str, amount: i64) -> Result<FungibleToken, CommonError>;
    async fn burn(&self, token_address: &str, requester_wallet: &Wallet, amount: i64) -> Result<FungibleToken, CommonError>;
}
