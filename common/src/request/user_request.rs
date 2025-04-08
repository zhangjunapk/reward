use entity::user::ActiveModel;
use sea_orm::ActiveValue;
use sea_orm::sqlx::types::chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRequest {
    pub service_provider_id: i64,
    pub out_user_id: i32,
    pub id: Option<i64>,
}

impl Into<ActiveModel> for UserRequest {
    fn into(self) -> ActiveModel {
        ActiveModel {
            service_provider_id: ActiveValue::Set(self.service_provider_id),
            out_user_id: ActiveValue::Set(self.out_user_id),
            create_time: ActiveValue::Set(Utc::now().naive_utc()),
            id: match self.id {
                Some(id) => ActiveValue::Set(id),
                None => ActiveValue::NotSet,
            },
        }
    }
}
