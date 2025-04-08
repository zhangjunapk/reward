use crate::DB;
use common::operate_receipt::OperateReceipt;
use entity::user;
use entity::user::{ActiveModel, Model};
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, PaginatorTrait, QueryFilter};

pub struct User;

type EntityUser = entity::user::Model;

impl User {
    pub async fn save(user: ActiveModel) -> Result<i64, OperateReceipt> {
        let result = user::Entity::insert(user.clone())
            .exec(DB.get().unwrap())
            .await
            .unwrap()
            .last_insert_id;
        Ok(result.into())
    }

    pub async fn find_service_user(service_provider_id:i64,out_user_id:i32)->Result<Option<Model>, DbErr> {
        user::Entity::find()
            .filter(
                Condition::any().add(
                    Condition::all()
                        .add(
                            user::Column::ServiceProviderId
                                .eq(service_provider_id),
                        )
                        .add(user::Column::OutUserId.eq(out_user_id)),
                ),
            )
            .one(DB.get().unwrap())
            .await
    }

    pub async fn find_one(id: i64) -> Result<Option<Model>, DbErr> {
        user::Entity::find_by_id(id).one(DB.get().unwrap()).await
    }

    pub async fn find_by_service(
        service_provider_id: i64,
        out_user_id: i32,
    ) -> Result<Option<Model>, DbErr> {
        user::Entity::find()
            .filter(
                Condition::all()
                    .add(user::Column::OutUserId.eq(out_user_id))
                    .add(user::Column::ServiceProviderId.eq(service_provider_id)),
            )
            .one(DB.get().unwrap())
            .await
    }
}
