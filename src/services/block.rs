use chrono::prelude::*;
use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::block::{Block, CreateBlock};
use crate::domain::repositories::block::{BlockQueryParams, BlockRepository};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::transaction::TransactionQueryParams;
use crate::domain::services::block::BlockService;
use crate::domain::services::transaction::TransactionService;
use crate::domain::services::wallet::WalletService;
use crate::utils::hex_utils::generate_block_hash;

#[derive(Clone)]
pub struct BlockServiceImpl {
    pub repository: Arc<dyn BlockRepository>,
    pub transaction_service: Arc<dyn TransactionService>,
    pub wallet_service: Arc<dyn WalletService>,
}

impl BlockServiceImpl {
    pub fn new(repository: Arc<dyn BlockRepository>, transaction_service: Arc<dyn TransactionService>, wallet_service: Arc<dyn WalletService>) -> Self {
        BlockServiceImpl {
            repository,
            transaction_service,
            wallet_service,
        }
    }
}

#[async_trait]
impl BlockService for BlockServiceImpl {
    async fn create(&self, miner_address: &str) -> Result<Block, CommonError> {
        let transaction_query_params = TransactionQueryParams {
            limit: Some(2),
            offset: Some(0),
            is_mined: Some(false),
        };

        let block_query_params = BlockQueryParams { limit: Some(1), offset: Some(0) };

        let raw_transactions = self.transaction_service.list(transaction_query_params).await?;

        let parent_block = self.list(block_query_params).await?; // Get last mined block

        let mut new_block = CreateBlock {
            block_hash: generate_block_hash(),
            parent_hash: if parent_block.items.is_empty() {
                "0x000000000000000000000000000000000000000000000000000000000GENESIS".to_string()
            } else {
                parent_block.items[0].block_hash.clone()
            },
            miner_address: miner_address.to_string(),
            timestamp: Some(Utc::now().naive_utc()),
            transaction_count: raw_transactions.items.len() as i32,
        };

        let created_block = self.repository.create(&mut new_block).await.map_err(|e| -> CommonError { e.into() })?;

        // Update the transactions
        for txn in raw_transactions.items {
            self.transaction_service.execute(created_block.block_number, &txn).await?;
        }
        Ok(created_block)
    }

    async fn list(&self, params: BlockQueryParams) -> Result<ResultPaging<Block>, CommonError> {
        self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, block_number: i32) -> Result<Block, CommonError> {
        self.repository.get(block_number).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, block_number: i32) -> Result<(), CommonError> {
        self.repository.delete(block_number).await.map_err(|e| -> CommonError { e.into() })
    }
}
