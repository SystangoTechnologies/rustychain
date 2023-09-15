use crate::domain::models::transaction::{CreateTransaction, Transaction, TransactionStatus, UpdateTransaction};
use crate::domain::models::transaction_type::TransactionType;
use crate::infrastructure::schema::transactions;
use diesel;
use diesel::prelude::*;
use serde_json::Value;

#[derive(Queryable)]
pub struct TransactionDiesel {
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

// Factory method for creating a new TransactionDiesel from a Transaction
impl From<Transaction> for TransactionDiesel {
    fn from(t: Transaction) -> Self {
        TransactionDiesel {
            id: t.id,
            block_number: t.block_number,
            transaction_hash: t.transaction_hash,
            from_address: t.from_address,
            to_address: t.to_address,
            transaction_type: t.transaction_type.as_str().to_string(),
            value: t.value,
            timestamp: t.timestamp,
            data: t.data,
            is_mined: t.is_mined,
            status: t.status.to_string(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct CreateTransactionDiesel {
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub transaction_type: String,
    pub value: i64,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub data: Option<Value>,
    pub status: String,
}

// Factory method for creating a new Transaction from a TransactionDiesel
impl Into<Transaction> for TransactionDiesel {
    fn into(self) -> Transaction {
        Transaction {
            id: self.id,
            block_number: self.block_number,
            transaction_hash: self.transaction_hash,
            from_address: self.from_address,
            to_address: self.to_address,
            transaction_type: TransactionType::from_str(&self.transaction_type),
            value: self.value,
            timestamp: self.timestamp,
            data: self.data,
            is_mined: self.is_mined,
            status: self.status.parse::<TransactionStatus>().unwrap_or(TransactionStatus::RAW),
        }
    }
}

impl From<CreateTransaction> for CreateTransactionDiesel {
    fn from(t: CreateTransaction) -> Self {
        CreateTransactionDiesel {
            transaction_hash: t.transaction_hash,
            from_address: t.from_address,
            to_address: t.to_address,
            transaction_type: t.transaction_type.as_str().to_string(),
            value: t.value,
            timestamp: t.timestamp,
            data: t.data,
            status: TransactionStatus::RAW.to_string(),
        }
    }
}

impl Into<CreateTransaction> for CreateTransactionDiesel {
    fn into(self) -> CreateTransaction {
        CreateTransaction {
            transaction_hash: self.transaction_hash,
            from_address: self.from_address,
            to_address: self.to_address,
            transaction_type: TransactionType::from_str(&self.transaction_type),
            value: self.value,
            timestamp: self.timestamp,
            data: self.data,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = transactions)]
pub struct UpdateTransactionDiesel {
    pub block_number: Option<i32>,
    pub is_mined: Option<bool>,
    pub status: Option<String>,
}

impl From<UpdateTransaction> for UpdateTransactionDiesel {
    fn from(update: UpdateTransaction) -> Self {
        UpdateTransactionDiesel {
            block_number: update.block_number,
            is_mined: update.is_mined,
            status: Some(update.status.unwrap().to_string()),
        }
    }
}
