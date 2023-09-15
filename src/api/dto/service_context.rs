use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::service_context::ServiceContext;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ServiceContextDTO {
    pub maintenance: bool,
}

impl Into<ServiceContextDTO> for ServiceContext {
    fn into(self) -> ServiceContextDTO {
        ServiceContextDTO { maintenance: self.maintenance }
    }
}

impl Into<ServiceContext> for ServiceContextDTO {
    fn into(self) -> ServiceContext {
        ServiceContext { id: 1, maintenance: self.maintenance }
    }
}
