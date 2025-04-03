use sea_orm::DeriveEntityModel;
use time::Time;

pub enum RewardStatus {
    Pending,
    Completed,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_reward")]
pub struct UserReward {
    #[sea_orm(primary_key)]
    id: i32,
    fee_user_id: i32,
    reward_user_id: i32,
    reward: f32,
    status:RewardStatus,
    create_time: Time,
}