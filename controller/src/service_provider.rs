use common::request::service_provider_request::ServiceProviderRequest;
use actix_web::dev::HttpServiceFactory;
use actix_web::{post, web};
use common::global_response::GlobalResponse;
use common::operate_receipt::OperateReceipt;
use common::response;
use entity::service_provider::Model;
use std::default::Default;

type ServiceProviderService = service::service_provider::ServiceProvider;

#[post("/service_provider/save")]
async fn save(service_provider: web::Json<ServiceProviderRequest>) -> GlobalResponse<()> {
    response!(ServiceProviderService::save(service_provider.0.into()).await)
}

#[post("/service_provider/update")]
async fn update(service_provider: web::Json<ServiceProviderRequest>) -> GlobalResponse<Model> {
    if let None = service_provider.id {
        return GlobalResponse {
            operate_receipt: OperateReceipt::Error("请求id不能为空".to_string()),
            data: None,
        };
    }
    response!(ServiceProviderService::update(service_provider.0.into()).await)
}
