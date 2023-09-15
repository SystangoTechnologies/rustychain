use crate::domain::models::fungible_token::{FungibleToken, UpdatedFungibleToken};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FungibleTokenQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for FungibleTokenQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait FungibleTokenRepository: Send + Sync {
    async fn create(&self, new_token: &FungibleToken) -> RepositoryResult<FungibleToken>;
    async fn list(&self, params: FungibleTokenQueryParams) -> RepositoryResult<ResultPaging<FungibleToken>>;
    async fn get(&self, token_address: &str) -> RepositoryResult<FungibleToken>;
    async fn update(&self, token_address: &str, updated_token: UpdatedFungibleToken) -> RepositoryResult<FungibleToken>;
}
