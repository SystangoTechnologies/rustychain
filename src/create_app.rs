use crate::api::controllers::block_handler::{create_block_handler, get_block_handler, list_block_handler};
use crate::api::controllers::fungible_token::{get_ft_handler, list_ft_handler};
use crate::api::controllers::service_context_handler::{get_service_context_handler, update_service_context_handler};
use crate::api::controllers::transaction_handler::{create_transaction_handler, delete_transaction_handler, get_transaction_handler, list_transaction_handler};
use crate::api::controllers::wallet_handler::{get_wallet_handler, list_wallet_handler};

use crate::api::middleware::ServiceContextMaintenanceCheck;
use crate::container::Container;
use crate::open_api::ApiDoc;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error};

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub fn create_app() -> App<impl ServiceFactory<ServiceRequest, Response = ServiceResponse<impl MessageBody>, Config = (), InitError = (), Error = Error>> {
    let container = Container::new();
    let service_context_service = container.service_context_service.clone();
    let transaction_service = container.transaction_service.clone();
    let block_service = container.block_service.clone();
    let wallet_service = container.wallet_service.clone();
    let fungible_token_service = container.fungible_token_service.clone();
    let openapi = ApiDoc::openapi();

    App::new()
        .app_data(web::Data::from(service_context_service.clone()))
        .app_data(web::Data::from(transaction_service.clone()))
        .app_data(web::Data::from(block_service.clone()))
        .app_data(web::Data::from(wallet_service.clone()))
        .app_data(web::Data::from(fungible_token_service.clone()))
        .wrap(ServiceContextMaintenanceCheck)
        .service(
            web::scope("/api/transactions")
                .route("", web::post().to(create_transaction_handler))
                .route("", web::get().to(list_transaction_handler))
                .route("/{id}", web::get().to(get_transaction_handler))
                .route("/{id}", web::delete().to(delete_transaction_handler)),
        )
        .service(
            web::scope("/api/blocks")
                .route("", web::post().to(create_block_handler))
                .route("", web::get().to(list_block_handler))
                .route("/{id}", web::get().to(get_block_handler)),
        )
        .service(
            web::scope("/api/wallets")
                .route("", web::get().to(list_wallet_handler))
                .route("/{wallet_address}/{token_address}", web::get().to(get_wallet_handler)),
        )
        .service(
            web::scope("/api/fts")
                .route("", web::get().to(list_ft_handler))
                .route("/{token_address}", web::get().to(get_ft_handler)),
        )
        .service(
            web::scope("/admin")
                .route("/maintenance/status", web::post().to(update_service_context_handler))
                .route("/maintenance/status", web::get().to(get_service_context_handler)),
        )
        .service(Redoc::with_url("/redoc", openapi.clone()))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
        .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .wrap(Logger::default())
}
