use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Block {
    pub block_number: i32,
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub miner_address: String,
    pub transaction_count: i32,
}

#[derive(Clone)]
pub struct CreateBlock {
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub miner_address: String,
    pub transaction_count: i32,
}
