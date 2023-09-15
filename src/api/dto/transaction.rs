use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::models::transaction::{CreateTransaction, Transaction};
use crate::domain::models::transaction_type::TransactionType;
use crate::domain::repositories::repository::ResultPaging;
use crate::utils::hex_utils::generate_transaction_hash;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTransactionDTO {
    pub from_address: String,
    pub to_address: String,
    pub transaction_type: String,
    pub value: i64,
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionDTO {
    pub id: i32,
    pub block_number: Option<i32>,
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub transaction_type: String,
    pub value: i64,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub data: Option<Value>,
    pub is_mined: Option<bool>,
    pub status: String,
}

impl Into<TransactionDTO> for Transaction {
    fn into(self) -> TransactionDTO {
        TransactionDTO {
            id: self.id,
            block_number: self.block_number,
            transaction_hash: self.transaction_hash,
            from_address: self.from_address,
            to_address: self.to_address,
            transaction_type: self.transaction_type.as_str().to_string(),
            value: self.value,
            timestamp: self.timestamp,
            data: self.data,
            is_mined: self.is_mined,
            status: self.status.to_string(),
        }
    }
}

impl Into<CreateTransaction> for CreateTransactionDTO {
    fn into(self) -> CreateTransaction {
        CreateTransaction {
            transaction_hash: generate_transaction_hash(),
            from_address: self.from_address,
            to_address: self.to_address,
            transaction_type: TransactionType::from_str(&self.transaction_type),
            value: self.value,
            timestamp: Some(Utc::now().naive_utc()),
            data: self.data,
        }
    }
}

impl Into<CreateTransactionDTO> for CreateTransaction {
    fn into(self) -> CreateTransactionDTO {
        CreateTransactionDTO {
            from_address: self.from_address,
            to_address: self.to_address,
            transaction_type: self.transaction_type.as_str().to_string(),
            value: self.value,
            data: self.data,
        }
    }
}

impl Into<ResultPaging<TransactionDTO>> for ResultPaging<Transaction> {
    fn into(self) -> ResultPaging<TransactionDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|transaction| transaction.into()).collect(),
        }
    }
}
