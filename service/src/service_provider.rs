use common::operate_receipt::OperateReceipt;
use entity::service_provider::ActiveModel;

type ServiceProviderRepository = repositories::service_provider::ServiceProvider;

pub struct ServiceProvider;

impl ServiceProvider {
    pub async fn save(service_provider: ActiveModel) -> Result<(), OperateReceipt> {
        ServiceProviderRepository::save(service_provider).await
    }
}
