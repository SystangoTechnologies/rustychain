use crate::api::dto::transaction::{CreateTransactionDTO, TransactionDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::transaction::TransactionQueryParams;
use crate::domain::services::transaction::TransactionService;
use actix_web::{web, HttpResponse, Result};

#[utoipa::path(
    post,
    path = "/api/transactions",
    tag = "Transactions",
    request_body = CreateTransactionDTO,
    responses(
        (status = 200, description = "Transaction created successfully", body = CreateTransactionDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),        
    )
)]
pub async fn create_transaction_handler(transaction_service: web::Data<dyn TransactionService>, post_data: web::Json<CreateTransactionDTO>) -> Result<web::Json<TransactionDTO>, ApiError> {
    let transaction = transaction_service.create(post_data.into_inner().into()).await?;
    Ok(web::Json(transaction.into()))
}

#[utoipa::path(
    get,
    path = "/api/transactions",
    tag = "Transactions",
    responses(
        (status = 200, description = "List of Transactions returned successfully", body = [TransactionDTO]),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn list_transaction_handler(transaction_service: web::Data<dyn TransactionService>, params: web::Query<TransactionQueryParams>) -> Result<web::Json<ResultPaging<TransactionDTO>>, ApiError> {
    let selection = transaction_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

#[utoipa::path(
    get,
    path = "/api/transactions/{id}",
    tag = "Transactions",
    params(
        ("id", description = "Unique storage id of Transaction")
    ),
    responses(
        (status = 200, description = "Transaction found successfully", body = TransactionDTO),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn get_transaction_handler(transaction_service: web::Data<dyn TransactionService>, params: web::Path<String>) -> Result<web::Json<TransactionDTO>, ApiError> {
    let transaction = transaction_service.get(&params.into_inner()).await?;
    Ok(web::Json(transaction.into()))
}

#[utoipa::path(
    delete,
    path = "/api/transactions/{id}",
    tag = "Transactions",
    params(
        ("id", description = "Unique id of Transaction")
    ),
    responses(
        (status = 200, description = "Transaction found successfully"),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn delete_transaction_handler(transaction_service: web::Data<dyn TransactionService>, params: web::Path<i32>) -> Result<HttpResponse, ApiError> {
    transaction_service.delete(params.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
