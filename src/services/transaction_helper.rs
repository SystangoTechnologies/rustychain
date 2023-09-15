use serde_json::Value;

pub enum ValidationResult {
    Valid,
    Invalid(String), // String contains error message when validation fails
}

use crate::domain::models::{
    transaction::CreateTransaction,
    transaction_type::{BurnFt, InitFt, MintFt, TransactionType, TransferFt},
};

// Should check symbol and name are specified
fn validate_init_ft(data: &Value) -> ValidationResult {
    let metadata: InitFt = data.clone().into();
    let mut error_messages = String::new();

    if !is_non_empty_string(&metadata.symbol) {
        error_messages += "Symbol is missing or empty. ";
    }

    if !is_non_empty_string(&metadata.name) {
        error_messages += "Name is missing or empty. ";
    }

    if error_messages.is_empty() {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid(error_messages)
    }
}

// Should check who is minting, to whom is token being minted, the quantity and which token
fn validate_mint_ft(data: &Value, txn: &CreateTransaction) -> ValidationResult {
    let metadata: MintFt = data.clone().into();
    let mut error_messages = String::new();

    if !is_non_empty_string(&metadata.token_address) {
        error_messages += "Token address is missing or empty. ";
    }

    if !is_non_empty_string(&txn.from_address) {
        error_messages += "From address is missing or empty. ";
    }

    if !is_non_empty_string(&txn.to_address) {
        error_messages += "To address is missing or empty. ";
    }

    if txn.value <= 0 {
        error_messages += "Value must be greater than zero. ";
    }

    if error_messages.is_empty() {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid(error_messages)
    }
}

// Should check who is burning, which token is being burnt, the quantity and which token
fn validate_burn_ft(data: &Value, txn: &CreateTransaction) -> ValidationResult {
    let metadata: BurnFt = data.clone().into();
    let mut error_messages = String::new();

    if !is_non_empty_string(&metadata.token_address) {
        error_messages += "Token address is missing or empty. ";
    }

    if !is_non_empty_string(&txn.from_address) {
        error_messages += "From address is missing or empty. ";
    }

    if txn.value <= 0 {
        error_messages += "Value must be greater than zero. ";
    }

    if error_messages.is_empty() {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid(error_messages)
    }
}

// Should check who is transferring, which token is being transferred, the quantity and to whom is token being transferred
fn validate_transfer_ft(data: &Value, txn: &CreateTransaction) -> ValidationResult {
    let metadata: TransferFt = data.clone().into();
    let mut error_messages = String::new();

    if !is_non_empty_string(&metadata.token_address) {
        error_messages += "Token address is missing or empty. ";
    }

    if !is_non_empty_string(&txn.from_address) {
        error_messages += "From address is missing or empty. ";
    }

    if !is_non_empty_string(&txn.to_address) {
        error_messages += "To address is missing or empty. ";
    }

    if txn.value <= 0 {
        error_messages += "Value must be greater than zero. ";
    }

    if error_messages.is_empty() {
        ValidationResult::Valid
    } else {
        ValidationResult::Invalid(error_messages)
    }
}

pub fn validate_transaction_metadata(txn: &CreateTransaction) -> ValidationResult {
    // check if data field contains a valid JSON
    if let Some(data) = txn.data.as_ref() {
        if data.is_object() {
            // validate JSON fields
            return match txn.transaction_type {
                TransactionType::InitFt => validate_init_ft(data),
                TransactionType::MintFt => validate_mint_ft(data, &txn),
                TransactionType::BurnFt => validate_burn_ft(data, &txn),
                TransactionType::TransferFt => validate_transfer_ft(data, &txn),
                TransactionType::InitNft => ValidationResult::Invalid("InitNft is not supported.".to_string()),
                TransactionType::MintNft => ValidationResult::Invalid("MintNft is not supported.".to_string()),
                TransactionType::BurnNft => ValidationResult::Invalid("BurnNft is not supported.".to_string()),
                TransactionType::TransferNft => ValidationResult::Invalid("TransferNft is not supported.".to_string()),
                TransactionType::None => ValidationResult::Invalid("TransactionType 'None' is not valid.".to_string()),
            };
        }
    }
    ValidationResult::Invalid("Invalid JSON data field.".to_string())
}

fn is_non_empty_string(input: &str) -> bool {
    !input.trim().is_empty()
}
