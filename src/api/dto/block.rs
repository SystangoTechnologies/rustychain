use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::block::Block;
use crate::domain::repositories::repository::ResultPaging;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockDTO {
    pub block_number: i32,
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub miner_address: String,
    pub transaction_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MineBlockDTO {
    pub miner_address: String,
}

impl Into<BlockDTO> for Block {
    fn into(self) -> BlockDTO {
        BlockDTO {
            block_number: self.block_number,
            block_hash: self.block_hash,
            parent_hash: self.parent_hash,
            timestamp: self.timestamp,
            miner_address: self.miner_address,
            transaction_count: self.transaction_count,
        }
    }
}

impl Into<ResultPaging<BlockDTO>> for ResultPaging<Block> {
    fn into(self) -> ResultPaging<BlockDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|block| block.into()).collect(),
        }
    }
}
