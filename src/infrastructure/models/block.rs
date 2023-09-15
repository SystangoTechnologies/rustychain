use crate::domain::models::block::{Block, CreateBlock};
use crate::infrastructure::schema::blocks;
use diesel;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct BlockDiesel {
    pub block_number: i32,
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub miner_address: String,
    pub transaction_count: i32,
}

impl From<Block> for BlockDiesel {
    fn from(t: Block) -> Self {
        BlockDiesel {
            block_number: t.block_number,
            block_hash: t.block_hash,
            parent_hash: t.parent_hash,
            timestamp: t.timestamp,
            miner_address: t.miner_address,
            transaction_count: t.transaction_count,
        }
    }
}

impl Into<Block> for BlockDiesel {
    fn into(self) -> Block {
        Block {
            block_number: self.block_number,
            block_hash: self.block_hash,
            parent_hash: self.parent_hash,
            timestamp: self.timestamp,
            miner_address: self.miner_address,
            transaction_count: self.transaction_count,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = blocks)]
pub struct CreateBlockDiesel {
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub miner_address: String,
    pub transaction_count: i32,
}

impl From<CreateBlock> for CreateBlockDiesel {
    fn from(t: CreateBlock) -> Self {
        CreateBlockDiesel {
            block_hash: t.block_hash,
            parent_hash: t.parent_hash,
            timestamp: t.timestamp,
            miner_address: t.miner_address,
            transaction_count: t.transaction_count,
        }
    }
}

impl Into<CreateBlock> for CreateBlockDiesel {
    fn into(self) -> CreateBlock {
        CreateBlock {
            block_hash: self.block_hash,
            parent_hash: self.parent_hash,
            timestamp: self.timestamp,
            miner_address: self.miner_address,
            transaction_count: self.transaction_count,
        }
    }
}
