use crate::api::dto::wallet::WalletDTO;
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::wallet::WalletQueryParams;
use crate::domain::services::wallet::WalletService;
use actix_web::{web, Result};

#[utoipa::path(
    get,
    path = "/api/wallets",
    tag = "Wallets",
    responses(
        (status = 200, description = "Wallets retrieved successfully", body = [WalletDTO]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
    )]
pub async fn list_wallet_handler(wallet_service: web::Data<dyn WalletService>, params: web::Query<WalletQueryParams>) -> Result<web::Json<ResultPaging<WalletDTO>>, ApiError> {
    let selection = wallet_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

#[utoipa::path(
    get,
    path = "/api/wallets/{wallet_address}/{token_address}",
    tag = "Wallets",
    params(
        ("wallet_address", description = "Owner Wallet address"),
        ("token_address", description = "Fungible Token address"),
    ),
    responses(
        (status = 200, description = "Wallet retrieved successfully", body = WalletDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn get_wallet_handler(wallet_service: web::Data<dyn WalletService>, param: web::Path<(String, String)>) -> Result<web::Json<WalletDTO>, ApiError> {
    let wallet = wallet_service.get(&param.0, &param.1).await?;
    Ok(web::Json(wallet.into()))
}
