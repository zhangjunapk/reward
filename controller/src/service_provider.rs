use crate::request::service_provider_request::ServiceProviderRequest;
use actix_web::{post, web};
use common::global_response::GlobalResponse;
use common::response;

type ServiceProviderService = service::service_provider::ServiceProvider;

#[post("/service_provider/save")]
async fn save(service_provider: web::Json<ServiceProviderRequest>) -> GlobalResponse<()> {
    response!(ServiceProviderService::save(service_provider.0.into()).await)
}
