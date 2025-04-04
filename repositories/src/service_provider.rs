use crate::DB;
use entity::service_provider;
use entity::service_provider::ActiveModel;

use common::operate_receipt::OperateReceipt;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Insert, PaginatorTrait, QueryFilter,
};
use std::fmt::Debug;
use std::ops::Deref;

pub struct ServiceProvider;

impl ServiceProvider {
    pub async fn save(server_provider: ActiveModel) -> Result<(), OperateReceipt> {
        let name = server_provider.name.clone().unwrap();
        let count = service_provider::Entity::find()
            .filter(Condition::all().add(service_provider::Column::Name.eq(name)))
            .count(DB.get().unwrap())
            .await
            .unwrap();
        println!("已经存在的数量:{}", count);
        if count > 0 {
            log::info!("服务商已存在");
            return Err(OperateReceipt::Exists("服务商已存在".to_string()));
        }
        service_provider::Entity::insert(server_provider)
            .exec(DB.get().unwrap())
            .await
            .unwrap();
        Ok(())
    }
}
