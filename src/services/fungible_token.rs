use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::fungible_token::{FungibleToken, UpdatedFungibleToken};
use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::fungible_token::{FungibleTokenQueryParams, FungibleTokenRepository};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::fungible_token::FungibleTokenService;

#[derive(Clone)]
pub struct FungibleTokenServiceImpl {
    pub repository: Arc<dyn FungibleTokenRepository>,
}

impl FungibleTokenServiceImpl {
    pub fn new(repository: Arc<dyn FungibleTokenRepository>) -> Self {
        FungibleTokenServiceImpl { repository }
    }
}

#[async_trait]
impl FungibleTokenService for FungibleTokenServiceImpl {
    async fn create(&self, create_fungible_token: FungibleToken) -> Result<FungibleToken, CommonError> {
        let created_fungible_token = self.repository.create(&create_fungible_token).await.map_err(|e| -> CommonError { e.into() })?;

        Ok(created_fungible_token)
    }

    async fn get(&self, address: &str) -> Result<FungibleToken, CommonError> {
        self.repository.get(address).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: FungibleTokenQueryParams) -> Result<ResultPaging<FungibleToken>, CommonError> {
        self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn mint(&self, token_address: &str, requester_address: &str, amount: i64) -> Result<FungibleToken, CommonError> {
        let token: FungibleToken = self.get(token_address).await?;
        if requester_address != token.owner_address {
            return Err(CommonError {
                message: String::from("Only owner can mint token"),
                code: 2,
            });
        }
        let updated_token = UpdatedFungibleToken {
            total_supply: Some(token.total_supply + amount),
        };

        self.repository.update(token_address, updated_token).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn burn(&self, token_address: &str, requester_wallet: &Wallet, amount: i64) -> Result<FungibleToken, CommonError> {
        let token: FungibleToken = self.get(token_address).await?;
        let pre_conditions: bool = requester_wallet.token_address == token_address && requester_wallet.balance >= amount;
        if !pre_conditions {
            return Err(CommonError {
                message: String::from("Requester does not have enough balance"),
                code: 2,
            });
        }
        let pre_conditions: bool = token.total_supply >= amount;
        if !pre_conditions {
            return Err(CommonError {
                message: String::from("Insufficient token balance to burn"),
                code: 2,
            });
        }
        let updated_token = UpdatedFungibleToken {
            total_supply: Some(token.total_supply - amount),
        };

        self.repository.update(token_address, updated_token).await.map_err(|e| -> CommonError { e.into() })
    }
}
