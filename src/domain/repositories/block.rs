use crate::domain::models::block::{Block, CreateBlock};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for BlockQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait BlockRepository: Send + Sync {
    async fn create(&self, new_block: &CreateBlock) -> RepositoryResult<Block>;
    async fn list(&self, params: BlockQueryParams) -> RepositoryResult<ResultPaging<Block>>;
    async fn get(&self, block_id: i32) -> RepositoryResult<Block>;
    async fn delete(&self, block_id: i32) -> RepositoryResult<()>;
}
