use utoipa::OpenApi;

use crate::api::dto::block::{BlockDTO, MineBlockDTO};
use crate::api::dto::fungible_token::FungibleTokenDTO;
use crate::api::dto::service_context::ServiceContextDTO;
use crate::api::dto::transaction::{CreateTransactionDTO, TransactionDTO};
use crate::api::dto::wallet::WalletDTO;

use crate::api::controllers::block_handler::{__path_create_block_handler, __path_get_block_handler, __path_list_block_handler};
use crate::api::controllers::fungible_token::{__path_get_ft_handler, __path_list_ft_handler};
use crate::api::controllers::service_context_handler::{__path_get_service_context_handler, __path_update_service_context_handler};
use crate::api::controllers::transaction_handler::{__path_create_transaction_handler, __path_get_transaction_handler, __path_list_transaction_handler};
use crate::api::controllers::wallet_handler::{__path_get_wallet_handler, __path_list_wallet_handler};

#[derive(OpenApi)]
#[openapi(
        paths(
            get_service_context_handler, update_service_context_handler,
            create_transaction_handler, list_transaction_handler, get_transaction_handler,
            create_block_handler, get_block_handler, list_block_handler,
            get_wallet_handler, list_wallet_handler,
            get_ft_handler, list_ft_handler,
            ),
        components(
            schemas(CreateTransactionDTO, TransactionDTO, MineBlockDTO, BlockDTO, FungibleTokenDTO, ServiceContextDTO, WalletDTO)
        ),
        tags(
            (name = "Rusty-Chain", description = "Rusty Chain management endpoints.")
        )
    )]
pub struct ApiDoc;
