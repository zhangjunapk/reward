use entity::service_provider::{ActiveModel, Status};
use sea_orm::ActiveValue;
use sea_orm::sqlx::types::chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServiceProviderRequest {
    pub name: String,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub status: Option<Status>,
    pub reward_config:String
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
            status: ActiveValue::Set(self.status.unwrap_or_else(|| Status::Enabled)),
            reward_config: ActiveValue::Set(self.reward_config.into()),
        }
    }
}
