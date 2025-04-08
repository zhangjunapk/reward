use crate::user_reward;
use common::operate_receipt::OperateReceipt;
use common::request::user_fee_request::UserFeeRequest;
use entity::service_provider::Status;
use entity::user_fee::ActiveModel;
use sea_orm::{ActiveValue, DbErr, InsertResult};
use sea_orm::sqlx::types::chrono::Utc;

type UserRepositories = repositories::user::User;
type UserFeeRepositories = repositories::user_fee::UserFee;

type UserRewardService = user_reward::UserReward;

type ServiceProviderRepositories = repositories::service_provider::ServiceProvider;

pub struct UserFee;

impl UserFee {
    pub async fn save(user_fee_request: UserFeeRequest) -> Result<(), OperateReceipt> {
        let UserFeeRequest {
            service_provider_id,
            out_user_id,
            fee,
        } = user_fee_request;

        let service_provider = ServiceProviderRepositories::find_one_by_id(service_provider_id)
            .await
            .map(|a| a.unwrap())?;
        if (service_provider.status == Status::Disabled) {
            return Err(OperateReceipt::Error("已禁用".to_string()));
        }
        let user = UserRepositories::find_service_user(service_provider_id, out_user_id).await;
        match user {
            Ok(user) => match user {
                None => Err(OperateReceipt::Error("找不到用户".to_string())),
                Some(user) => {
                    let value = ActiveModel {
                        user_id: ActiveValue::Set(user.id),
                        fee: ActiveValue::Set(fee),
                        create_time: ActiveValue::Set(Utc::now().naive_utc()),
                        id: Default::default(),
                    };
                    let fee_id  = UserFeeRepositories::save(value)
                        .await
                        .map(|a|a.last_insert_id)
                        .map_err(|db_err|OperateReceipt::Error(db_err.to_string()))?;

                    UserRewardService::reward(fee_id,service_provider.reward_config(),user_fee_request).await
                }
            },
            Err(error) => Err(OperateReceipt::Error(error.to_string())),
        }
    }
}
