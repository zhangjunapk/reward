use crate::DB;
use entity::service_provider;
use entity::service_provider::{ActiveModel, Model};

use common::operate_receipt::OperateReceipt;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};
use std::fmt::Debug;

pub struct ServiceProvider;

impl ServiceProvider {
    pub async fn find_one_by_name(name: String) -> Result<Option<Model>, OperateReceipt> {
        service_provider::Entity::find()
            .filter(service_provider::Column::Name.eq(name))
            .one(DB.get().unwrap())
            .await
            .map_err(|db_err: DbErr| OperateReceipt::Error(db_err.to_string()))
    }

    pub async fn find_one_by_id(id: i64) -> Result<Option<Model>, OperateReceipt> {
        service_provider::Entity::find()
            .filter(service_provider::Column::Id.eq(id))
            .one(DB.get().unwrap())
            .await
            .map_err(|db_err: DbErr| OperateReceipt::Error(db_err.to_string()))
    }

    pub async fn save(server_provider: ActiveModel) -> Result<(), OperateReceipt> {
        service_provider::Entity::insert(server_provider)
            .exec(DB.get().unwrap())
            .await
            .unwrap();
        Ok(())
    }
    pub async fn update(service_provider: ActiveModel) -> Result<Model, OperateReceipt> {
        service_provider::Entity::update(service_provider)
            .exec(DB.get().unwrap())
            .await
            .and_then(|model| Ok(model))
            .map_err(|op| OperateReceipt::Error(op.to_string()))
    }
}
