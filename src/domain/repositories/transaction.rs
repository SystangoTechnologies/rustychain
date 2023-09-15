use crate::domain::models::transaction::{CreateTransaction, Transaction, UpdateTransaction};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub is_mined: Option<bool>,
}

impl QueryParams for TransactionQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, new_transaction: &CreateTransaction) -> RepositoryResult<Transaction>;
    async fn list(&self, params: TransactionQueryParams) -> RepositoryResult<ResultPaging<Transaction>>;
    async fn get(&self, transaction_hash: &str) -> RepositoryResult<Transaction>;
    async fn delete(&self, transaction_id: i32) -> RepositoryResult<()>;
    async fn update(&self, transaction_id: i32, update_data: UpdateTransaction) -> RepositoryResult<Transaction>;
}
