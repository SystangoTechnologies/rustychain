use crate::domain::repositories::block::BlockRepository;
use crate::domain::repositories::fungible_token::FungibleTokenRepository;
use crate::domain::repositories::transaction::TransactionRepository;
use crate::domain::repositories::wallet::WalletRepository;
use crate::domain::services::block::BlockService;
use crate::domain::services::fungible_token::FungibleTokenService;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::transaction::TransactionService;
use crate::domain::services::wallet::WalletService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::block::BlockDieselRepository;
use crate::infrastructure::repositories::fungible_token::FungibleTokenDieselRepository;
use crate::infrastructure::repositories::transaction::TransactionDieselRepository;
use crate::infrastructure::repositories::wallet::WalletDieselRepository;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::block::BlockServiceImpl;
use crate::services::fungible_token::FungibleTokenServiceImpl;
use crate::services::transaction::TransactionServiceImpl;
use crate::services::wallet::WalletServiceImpl;
use std::sync::Arc;

pub struct Container {
    pub service_context_service: Arc<dyn ServiceContextService>,
    pub transaction_service: Arc<dyn TransactionService>,
    pub block_service: Arc<dyn BlockService>,
    pub wallet_service: Arc<dyn WalletService>,
    pub fungible_token_service: Arc<dyn FungibleTokenService>,
}

impl Container {
    pub fn new() -> Self {
        let db_pool = Arc::new(db_pool()); // Create the database pool only once

        let service_context_service = Arc::new(ServiceContextServiceImpl::new(Arc::clone(&db_pool)));

        let wallet_repository: Arc<dyn WalletRepository> = Arc::new(WalletDieselRepository::new(Arc::clone(&db_pool)));

        let wallet_service = Arc::new(WalletServiceImpl { repository: wallet_repository });

        let fungible_token_repository: Arc<dyn FungibleTokenRepository> = Arc::new(FungibleTokenDieselRepository::new(Arc::clone(&db_pool)));

        let fungible_token_service = Arc::new(FungibleTokenServiceImpl {
            repository: fungible_token_repository,
        });

        let transaction_repository: Arc<dyn TransactionRepository> = Arc::new(TransactionDieselRepository::new(Arc::clone(&db_pool)));

        let transaction_service = Arc::new(TransactionServiceImpl {
            repository: transaction_repository,
            wallet_service: wallet_service.clone(),
            fungible_token_service: fungible_token_service.clone(),
        });

        let block_repository: Arc<dyn BlockRepository> = Arc::new(BlockDieselRepository::new(Arc::clone(&db_pool)));

        let block_service = Arc::new(BlockServiceImpl {
            repository: block_repository,
            transaction_service: transaction_service.clone(),
            wallet_service: wallet_service.clone(),
        });

        Container {
            service_context_service,
            transaction_service,
            block_service,
            wallet_service,
            fungible_token_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
