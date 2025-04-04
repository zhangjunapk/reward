use entity::service_provider::ActiveModel;
use sea_orm::ActiveValue;
use sea_orm::sqlx::types::chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServiceProviderRequest {
    pub name: String,
    pub description: Option<String>,
    pub id: Option<i64>,
}

impl Into<ActiveModel> for ServiceProviderRequest {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: match self.id {
                Some(id) => ActiveValue::Set(id),
                None => ActiveValue::NotSet,
            },
            name: ActiveValue::Set(self.name.clone()),
            description: ActiveValue::Set(self.description.clone()),
            create_time: ActiveValue::Set(Utc::now().naive_utc()),
        }
    }
}
