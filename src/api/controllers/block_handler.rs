use crate::api::dto::block::{BlockDTO, MineBlockDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::block::BlockQueryParams;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::block::BlockService;
use actix_web::{web, Result};

#[utoipa::path(
    post,
    path = "/api/blocks",
    tag = "Blocks",
    request_body = MineBlockDTO,
    responses(
        (status = 200, description = "Block mined successfully", body = BlockDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn create_block_handler(block_service: web::Data<dyn BlockService>, post_data: web::Json<MineBlockDTO>) -> Result<web::Json<BlockDTO>, ApiError> {
    let miner_address = &post_data.miner_address;
    let block = block_service.create(miner_address).await?;
    Ok(web::Json(block.into()))
}

#[utoipa::path(
    get,
    path = "/api/blocks",
    tag = "Blocks",
    responses(
        (status = 200, description = "Blocks retrieved successfully", body = [BlockDTO]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn list_block_handler(block_service: web::Data<dyn BlockService>, params: web::Query<BlockQueryParams>) -> Result<web::Json<ResultPaging<BlockDTO>>, ApiError> {
    let selection = block_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

#[utoipa::path(
    get,
    path = "/api/blocks/{id}",
    tag = "Blocks",
    params(
        ("id", description = "Block Height")
    ),
    responses(
        (status = 200, description = "Block found successfully", body = BlockDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn get_block_handler(block_service: web::Data<dyn BlockService>, params: web::Path<i32>) -> Result<web::Json<BlockDTO>, ApiError> {
    let block = block_service.get(params.into_inner()).await?;
    Ok(web::Json(block.into()))
}
