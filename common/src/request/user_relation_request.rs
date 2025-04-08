use entity::user_relation::ActiveModel;
use sea_orm::ActiveValue;
use sea_orm::sqlx::types::chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRelationRequest {
    pub service_provider_id:i64,
    pub parent_id: i64,
    pub user_id: i64,
}

impl Into<ActiveModel> for UserRelationRequest {
    fn into(self) ->ActiveModel {
        ActiveModel {
            parent_id: ActiveValue::Set(self.parent_id),
            left: ActiveValue::Set(1),
            right: ActiveValue::Set(2),
            create_time: ActiveValue::Set(Utc::now().naive_utc()),
            level: ActiveValue::Set(0),
            user_id: ActiveValue::Set(self.user_id),
            id: ActiveValue::NotSet,
            service_provider_id:ActiveValue::Set(self.service_provider_id)
        }.to_owned()
    }
}
