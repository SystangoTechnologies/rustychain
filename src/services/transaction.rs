use actix_web::Result;
use async_trait::async_trait;
use std::sync::Arc;

use super::transaction_helper::ValidationResult;
use crate::domain::error::CommonError;
use crate::domain::models::fungible_token::FungibleToken;
use crate::domain::models::transaction::{CreateTransaction, Transaction, TransactionStatus, UpdateTransaction};
use crate::domain::models::transaction_type::{BurnFt, InitFt, MintFt, TransactionType, TransferFt};
use crate::domain::models::wallet::Wallet;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::transaction::{TransactionQueryParams, TransactionRepository};
use crate::domain::services::fungible_token::FungibleTokenService;
use crate::domain::services::transaction::TransactionService;
use crate::domain::services::wallet::WalletService;
use crate::services::transaction_helper::validate_transaction_metadata;
use crate::utils::hex_utils::generate_hex_address;

#[derive(Clone)]
pub struct TransactionServiceImpl {
    pub repository: Arc<dyn TransactionRepository>,
    pub wallet_service: Arc<dyn WalletService>,
    pub fungible_token_service: Arc<dyn FungibleTokenService>,
}

impl TransactionServiceImpl {
    pub fn new(repository: Arc<dyn TransactionRepository>, wallet_service: Arc<dyn WalletService>, fungible_token_service: Arc<dyn FungibleTokenService>) -> Self {
        TransactionServiceImpl {
            repository,
            wallet_service,
            fungible_token_service,
        }
    }

    async fn handle_ft_init(&self, block_number: i32, txn: &Transaction) -> Result<(), CommonError> {
        let data = txn.data.as_ref().unwrap();
        let metadata: InitFt = data.clone().into();

        // Step-1 : create the token in the database
        let fungible_token = FungibleToken {
            address: generate_hex_address(),
            symbol: metadata.symbol.clone(),
            name: metadata.name.clone(),
            owner_address: txn.from_address.clone(),
            decimals: metadata.decimals,
            total_supply: txn.value.into(),
            block_number: block_number,
            transaction_hash: txn.transaction_hash.clone(),
        };
        let new_token = self.fungible_token_service.create(fungible_token).await?;

        // Step-2 : create the wallet for the owner and credit the initial supply
        let new_wallet = Wallet {
            address: txn.from_address.clone(),
            token_address: new_token.address.clone(),
            balance: txn.value,
            block_number: block_number,
            transaction_hash: txn.transaction_hash.clone(),
        };
        self.wallet_service.create_or_update(new_wallet).await?;
        Ok(())
    }

    async fn handle_ft_transfer(&self, block_number: i32, txn: &Transaction) -> Result<(), CommonError> {
        let data = txn.data.as_ref().unwrap();
        let metadata: TransferFt = data.clone().into();

        // Step-1 : update the balance of the from_wallet
        let mut from_wallet = self.wallet_service.get(&txn.from_address, &metadata.token_address).await?;
        if from_wallet.balance >= txn.value {
            from_wallet.balance -= txn.value;
            from_wallet.block_number = block_number;
            from_wallet.transaction_hash = txn.transaction_hash.clone();

            // Step-2 : update the balance of the to_wallet
            let to_wallet_result = self.wallet_service.get(&txn.to_address, &metadata.token_address).await;
            // Get or create from_wallet
            let to_wallet = match to_wallet_result {
                Ok(existing_wallet) => {
                    // Update the balance
                    Wallet {
                        address: txn.to_address.clone(),
                        token_address: metadata.token_address.clone(),
                        balance: existing_wallet.balance + txn.value,
                        block_number: block_number,
                        transaction_hash: txn.transaction_hash.clone(),
                    }
                }
                Err(_) => {
                    // Create a new Wallet instance
                    Wallet {
                        address: txn.to_address.clone(),
                        token_address: metadata.token_address.clone(),
                        balance: txn.value,
                        block_number: block_number,
                        transaction_hash: txn.transaction_hash.clone(),
                    }
                }
            };

            // Step-3 : persist the updated wallets
            self.wallet_service.create_or_update(from_wallet).await?;
            self.wallet_service.create_or_update(to_wallet).await?;
            Ok(())
        } else {
            Err(CommonError {
                message: "Insufficient balance in sender's wallet".to_string(),
                code: 1,
            })
        }
    }

    async fn handle_ft_mint(&self, block_number: i32, txn: &Transaction) -> Result<(), CommonError> {
        let data = txn.data.as_ref().unwrap();
        let metadata: MintFt = data.clone().into();

        // Step-1 : mint the new token
        self.fungible_token_service.mint(&metadata.token_address, &txn.from_address, txn.value).await?;

        // Step-2 : if mint was successful, update the balance of the to_wallet
        let to_wallet_result = self.wallet_service.get(&txn.to_address, &metadata.token_address).await;
        let mint_to_wallet = match to_wallet_result {
            Ok(existing_wallet) => {
                // Update the balance
                Wallet {
                    address: txn.to_address.clone(),
                    token_address: metadata.token_address.clone(),
                    balance: existing_wallet.balance + txn.value,
                    block_number: block_number,
                    transaction_hash: txn.transaction_hash.clone(),
                }
            }
            Err(_) => {
                // Create a new Wallet instance
                Wallet {
                    address: txn.to_address.clone(),
                    token_address: metadata.token_address.clone(),
                    balance: txn.value,
                    block_number: block_number,
                    transaction_hash: txn.transaction_hash.clone(),
                }
            }
        };

        // Step-3 : persist the updated wallet
        self.wallet_service.create_or_update(mint_to_wallet).await?;
        Ok(())
    }

    async fn handle_ft_burn(&self, block_number: i32, txn: &Transaction) -> Result<(), CommonError> {
        let data = txn.data.as_ref().unwrap();
        let metadata: BurnFt = data.clone().into();

        // Step-1 : update the balance of the from_wallet
        let from_wallet = self.wallet_service.get(&txn.from_address, &metadata.token_address).await?;
        self.fungible_token_service.burn(&metadata.token_address, &from_wallet, txn.value).await?;
        let updated_wallet = Wallet {
            address: txn.from_address.clone(),
            token_address: metadata.token_address.clone(),
            balance: from_wallet.balance - txn.value,
            block_number: block_number,
            transaction_hash: txn.transaction_hash.clone(),
        };

        // Step-2 : persist the updated wallet
        self.wallet_service.create_or_update(updated_wallet).await?;
        Ok(())
    }
}

#[async_trait]
impl TransactionService for TransactionServiceImpl {
    async fn create(&self, transaction: CreateTransaction) -> Result<Transaction, CommonError> {
        let validation_result = validate_transaction_metadata(&transaction);
        if let ValidationResult::Invalid(error_messages) = validation_result {
            return Err(CommonError {
                message: format!("Invalid transaction metadata: {}", error_messages),
                code: 3,
            });
        }

        let mut cloned = transaction.clone();
        self.repository.create(&mut cloned).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: TransactionQueryParams) -> Result<ResultPaging<Transaction>, CommonError> {
        self.repository.list(params).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, transaction_hash: &str) -> Result<Transaction, CommonError> {
        self.repository.get(transaction_hash).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, transaction_id: i32) -> Result<(), CommonError> {
        self.repository.delete(transaction_id).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn update(&self, transaction_id: i32, update_data: UpdateTransaction) -> Result<Transaction, CommonError> {
        self.repository.update(transaction_id, update_data).await.map_err(|e| -> CommonError { e.into() })
    }

    async fn execute(&self, block_number: i32, txn: &Transaction) -> Result<(), CommonError> {
        let mut txn_status: TransactionStatus = TransactionStatus::SUCCESS;
        match txn.transaction_type {
            TransactionType::InitFt => {
                if let Err(_) = self.handle_ft_init(block_number, txn).await {
                    txn_status = TransactionStatus::FAIL;
                }
            }
            TransactionType::MintFt => {
                if let Err(_) = self.handle_ft_mint(block_number, txn).await {
                    txn_status = TransactionStatus::FAIL;
                }
            }
            TransactionType::BurnFt => {
                if let Err(_) = self.handle_ft_burn(block_number, txn).await {
                    txn_status = TransactionStatus::FAIL;
                }
            }
            TransactionType::TransferFt => {
                if let Err(_) = self.handle_ft_transfer(block_number, txn).await {
                    txn_status = TransactionStatus::FAIL;
                }
            }
            TransactionType::InitNft => {
                // Handle InitFt case
                txn_status = TransactionStatus::FAIL;
            }
            TransactionType::MintNft => {
                // Handle MintFt case
                txn_status = TransactionStatus::FAIL;
            }
            TransactionType::BurnNft => {
                // Handle BurnFt case
                txn_status = TransactionStatus::FAIL;
            }
            TransactionType::TransferNft => {
                // Handle TransferFt case
                txn_status = TransactionStatus::FAIL;
            }
            TransactionType::None => {
                txn_status = TransactionStatus::FAIL;
            }
        }

        // Update the transaction is_mined and block_number
        let updated_txn = UpdateTransaction {
            is_mined: Some(true),
            block_number: Some(block_number),
            status: Some(txn_status),
        };
        self.update(txn.id, updated_txn).await?;
        Ok(())
    }
}
