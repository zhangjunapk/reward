use crate::DB;
use entity::user_fee;
use entity::user_fee::ActiveModel;
use sea_orm::{DbErr, EntityTrait, InsertResult};

pub struct UserFee;
impl UserFee {
    pub async fn save(user_fee: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
        user_fee::Entity::insert(user_fee)
            .exec(DB.get().unwrap())
            .await
    }
}
