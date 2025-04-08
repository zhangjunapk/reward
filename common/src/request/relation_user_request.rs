use serde::Deserialize;

#[derive(Deserialize)]
pub struct RelationUserRequest{
    pub service_provider_id:i64,
    pub out_user_id:i32
}