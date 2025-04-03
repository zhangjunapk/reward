use sea_orm::DeriveEntityModel;
use time::Time;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "service_provider")]
pub struct ServiceProvider {
    #[sea_orm(primary_key)]
    id: i32,
    name: String,
    description: String,
    create_time: Time,
}
