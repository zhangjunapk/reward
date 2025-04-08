use crate::DB;
use entity::user_reward;
use entity::user_reward::ActiveModel;
use sea_orm::{DbErr, EntityTrait, InsertResult};

pub async fn save(user_reward: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
    user_reward::Entity::insert(user_reward)
        .exec(DB.get().unwrap())
        .await
}
