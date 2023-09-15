use crate::api::dto::fungible_token::FungibleTokenDTO;
use crate::domain::error::ApiError;
use crate::domain::repositories::fungible_token::FungibleTokenQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::fungible_token::FungibleTokenService;
use actix_web::{web, Result};

#[utoipa::path(
    get,
    path = "/api/fts",
    tag = "Fungible Token",
    responses(
        (status = 200, description = "Fungible Tokens retrieved successfully", body = [FungibleTokenDTO]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn list_ft_handler(ft_service: web::Data<dyn FungibleTokenService>, params: web::Query<FungibleTokenQueryParams>) -> Result<web::Json<ResultPaging<FungibleTokenDTO>>, ApiError> {
    let selection = ft_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

#[utoipa::path(
    get,
    path = "/api/fts/{address}",
    tag = "Fungible Token",
    params(
        ("address", description = "Fungible Token address")
    ),
    responses(
        (status = 200, description = "Fungible Token found successfully", body = FungibleTokenDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn get_ft_handler(ft_service: web::Data<dyn FungibleTokenService>, params: web::Path<String>) -> Result<web::Json<FungibleTokenDTO>, ApiError> {
    let ft = ft_service.get(&params.into_inner()).await?;
    Ok(web::Json(ft.into()))
}
