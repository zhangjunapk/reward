use common::operate_receipt::OperateReceipt;
use common::request::user_fee_request::UserFeeRequest;
use entity::service_provider::RewardConfig;
use entity::user_relation::Model;
use entity::user_reward;
use entity::user_reward::{ActiveModel, Status};
use repositories::DB;
use rust_decimal::Decimal;
use sea_orm::sqlx::types::chrono::Utc;
use sea_orm::{ActiveValue, EntityTrait};

type UserRelationRepositories = repositories::user_relation::UserRelation;

type UserRelationModel = Model;

pub struct UserReward;

impl UserReward {
    pub async fn reward(
        fee_id: i64,
        reward_config: RewardConfig,
        user_fee_request: UserFeeRequest,
    ) -> Result<(), OperateReceipt> {
        let user_relation = UserRelationRepositories::find_one_by_out(
            user_fee_request.service_provider_id,
            user_fee_request.out_user_id,
        )
        .await;
        match user_relation {
            Ok(user_relation) => {
                if let Some(user_relation) = user_relation {
                    let parent =
                        UserRelationRepositories::parent(user_relation.left, user_relation.right)
                            .await;
                    match parent {
                        Ok(parent) => {
                            Self::parent_reward(
                                fee_id,
                                user_relation,
                                reward_config,
                                user_fee_request,
                                parent.to_owned(),
                            )
                            .await
                        }
                        Err(db_err) => Err(OperateReceipt::Error(db_err.to_string())),
                    }
                } else {
                    Err(OperateReceipt::Error("找不到用户关系".to_string()))
                }
            }
            Err(db_err) => Err(OperateReceipt::Error(db_err.to_string())),
        }
    }
    async fn parent_reward(
        fee_id: i64,
        user_relation: UserRelationModel,
        reward_config: RewardConfig,
        user_fee_request: UserFeeRequest,
        mut parents: Vec<Model>,
    ) -> Result<(), OperateReceipt> {
        parents.sort_by(|a, b| a.level.cmp(&b.level));
        let should_insert = Self::reward_calculation(
            fee_id,
            reward_config,
            user_relation,
            user_fee_request,
            parents,
        )
        .await;

        for x in should_insert {
            let _ = user_reward::Entity::insert(x).exec(DB.get().unwrap()).await;
        }

        Ok(())
    }

    async fn reward_calculation(
        fee_id: i64,
        reward_config: RewardConfig,
        user_relation_model: UserRelationModel,
        user_fee_request: UserFeeRequest,
        mut parents: Vec<Model>,
    ) -> Vec<ActiveModel> {
        let mut models: Vec<ActiveModel> = vec![];
        let mut fee = user_fee_request.fee;
        match reward_config {
            RewardConfig::Default(root_percent) => {
                let mut should_fee;
                for i in 0..parents.len() {
                    let current_parent = parents.get(i);
                    match current_parent {
                        Some(current_parent) => {
                            if i == 0 {
                                if i == parents.len() - 1 {
                                    should_fee = fee;
                                } else {
                                    should_fee = fee * Decimal::new(root_percent as i64, 2);
                                }
                            } else if i == parents.len() - 1 {
                                should_fee = fee;
                            } else {
                                should_fee = fee * Decimal::new(50, 2);
                            }
                            fee = fee - should_fee;

                            models.push(ActiveModel {
                                fee_user_id: ActiveValue::Set(user_relation_model.user_id),
                                reward_user_id: ActiveValue::Set(current_parent.user_id),
                                fee_id: ActiveValue::Set(fee_id),
                                reward: ActiveValue::Set(should_fee),
                                create_time: ActiveValue::Set(Utc::now().naive_utc()),
                                status: ActiveValue::Set(Status::Pending),
                                id: ActiveValue::NotSet,
                            });
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
            RewardConfig::HalfSlice => {
                let mut should_fee = fee;
                for i in 0..parents.len() {
                    let item = parents.get(i).unwrap();
                    if i == parents.len() - 1 {
                        should_fee = fee;
                    } else {
                        should_fee = fee * Decimal::new(50, 2);
                    }
                    fee = fee - should_fee;
                    models.push(ActiveModel {
                        fee_user_id: ActiveValue::Set(user_relation_model.user_id),
                        reward_user_id: ActiveValue::Set(item.user_id),
                        fee_id: ActiveValue::Set(fee_id),
                        reward: ActiveValue::Set(should_fee),
                        create_time: ActiveValue::Set(Utc::now().naive_utc()),
                        status: ActiveValue::Set(Status::Pending),
                        id: ActiveValue::NotSet,
                    });
                }
            }
        }
        models
    }

    pub async fn success(){

    }

}
