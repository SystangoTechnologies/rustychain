use crate::api::dto::service_context::ServiceContextDTO;
use crate::domain::error::ApiError;
use crate::domain::services::service_context::ServiceContextService;
use actix_web::{web, Result};

#[utoipa::path(
    post,
    path = "/admin/maintenance/status",
    tag = "Service Context",
    request_body = ServiceContextDTO,
    responses(
        (status = 200, description = "Service Context updated successfully", body = ServiceContextDTO),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Service Context not found"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn update_service_context_handler(service_context_service: web::Data<dyn ServiceContextService>, post_data: web::Json<ServiceContextDTO>) -> Result<web::Json<ServiceContextDTO>, ApiError> {
    let service_context = service_context_service.update(post_data.into_inner().into());
    Ok(web::Json(service_context.into()))
}

#[utoipa::path(
    get,
    path = "/admin/maintenance/status",
    tag = "Service Context",
    responses(
        (status = 200, description = "Service Context retrieved successfully", body = ServiceContextDTO),
        (status = 404, description = "Service Context not found"),
        (status = 500, description = "Internal Server Error"),
        (status = 503, description = "Service Unavailable"),
        (status = 429, description = "Too Many Requests"),
    )
)]
pub async fn get_service_context_handler(service_context_service: web::Data<dyn ServiceContextService>) -> Result<web::Json<ServiceContextDTO>, ApiError> {
    let service_context = service_context_service.get_service_context();
    Ok(web::Json(service_context.into()))
}
