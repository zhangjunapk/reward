use sea_orm::DeriveEntityModel;
use time::Time;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_relation")]
pub struct UserRelation {
    #[sea_orm(primary_key)]
    id:i32,
    user_id,
    parent_id:i32,
    left:i32,
    right:i32,
    level:u8,
    create_time:Time
}