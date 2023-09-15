use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use std::str::FromStr;

use super::transaction_type::TransactionType;

#[derive(Clone, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub block_number: Option<i32>,
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub transaction_type: TransactionType,
    pub value: i64,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub data: Option<Value>,
    pub is_mined: Option<bool>,
    pub status: TransactionStatus,
}

#[derive(Clone, Debug, Deserialize)]
pub enum TransactionStatus {
    RAW,
    SUCCESS,
    FAIL,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TransactionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAW" => Ok(TransactionStatus::RAW),
            "SUCCESS" => Ok(TransactionStatus::SUCCESS),
            "FAIL" => Ok(TransactionStatus::FAIL),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTransaction {
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub transaction_type: TransactionType,
    pub value: i64,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub data: Option<Value>,
}

#[derive(Clone)]
pub struct UpdateTransaction {
    pub block_number: Option<i32>,
    pub is_mined: Option<bool>,
    pub status: Option<TransactionStatus>,
}
