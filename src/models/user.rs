use sea_orm::DeriveEntityModel;
use time::Time;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "service_provider")]
pub struct User {
    #[sea_orm(primary_key)]
    id: i32,
    service_provider_id: i32,
    out_user_id: i32,
    create_time: Time,
}
