use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::transaction::{CreateTransaction, Transaction, UpdateTransaction};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::transaction::TransactionQueryParams;

#[async_trait]
pub trait TransactionService: Sync + Send {
    async fn create(&self, transaction: CreateTransaction) -> Result<Transaction, CommonError>;
    async fn list(&self, params: TransactionQueryParams) -> Result<ResultPaging<Transaction>, CommonError>;
    async fn get(&self, transaction_hash: &str) -> Result<Transaction, CommonError>;
    async fn delete(&self, transaction_id: i32) -> Result<(), CommonError>;
    async fn update(&self, transaction_id: i32, update_data: UpdateTransaction) -> Result<Transaction, CommonError>;
    async fn execute(&self, block_number: i32, transaction: &Transaction) -> Result<(), CommonError>;
}
