//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_reward")]
pub struct Model {
    pub fee_user_id: i32,
    pub reward_user_id: i32,
    pub reward: Decimal,
    pub create_time: Time,
    pub status: String,
    #[sea_orm(primary_key)]
    pub id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
