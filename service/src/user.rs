use common::operate_receipt::OperateReceipt;
use entity::user::{ActiveModel, Model};

type UserRepository = repositories::user::User;

type ServiceProviderRepository = repositories::service_provider::ServiceProvider;

pub struct User;

impl User {
    pub async fn save(user: ActiveModel) -> Result<i64, OperateReceipt> {
        let one =
            ServiceProviderRepository::find_one_by_id(user.clone().service_provider_id.unwrap())
                .await;
        if let Ok(service_provider) = one {
            if service_provider.is_none() {
                return Err(OperateReceipt::Error("服务商不存在".to_string()));
            }
        }
        let find_user = UserRepository::find_by_service(
            user.clone().service_provider_id.unwrap(),
            user.clone().out_user_id.unwrap(),
        )
        .await;
        if let Ok(service_provider) = find_user {
            if service_provider.is_some() {
                return Err(OperateReceipt::Error("用户已存在".to_string()));
            }
        }
        UserRepository::save(user).await
    }
    pub async fn find_one(id: i64) -> Result<Option<Model>, OperateReceipt> {
        UserRepository::find_one(id)
            .await
            .map_err(|db_err| OperateReceipt::Error("操作出错".to_string()))
    }
}
