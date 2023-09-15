use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::block::Block;
use crate::domain::repositories::block::BlockQueryParams;
use crate::domain::repositories::repository::ResultPaging;

#[async_trait]
pub trait BlockService: Sync + Send {
    async fn create(&self, miner_address: &str) -> Result<Block, CommonError>;
    async fn list(&self, params: BlockQueryParams) -> Result<ResultPaging<Block>, CommonError>;
    async fn get(&self, block_id: i32) -> Result<Block, CommonError>;
    async fn delete(&self, block_id: i32) -> Result<(), CommonError>;
}
