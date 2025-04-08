use common::operate_receipt::OperateReceipt;
use entity::service_provider::{ActiveModel, Model};

type ServiceProviderRepository = repositories::service_provider::ServiceProvider;

pub struct ServiceProvider;

impl ServiceProvider {
    pub async fn save(service_provider: ActiveModel) -> Result<(), OperateReceipt> {
        ServiceProviderRepository::find_one_by_name(service_provider.clone().name.unwrap()).await?;
        ServiceProviderRepository::save(service_provider).await
    }

    pub async fn update(service_provider: ActiveModel) -> Result<Model, OperateReceipt> {
        ServiceProviderRepository::update(service_provider).await
    }
}
