use sea_orm::DeriveEntityModel;
use time::Time;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_fee")]
pub struct UserFee{
    #[sea_orm(primary_key)]
    id:i32,
    user_id:i32,
    fee:f32,
    create_time:Time
}