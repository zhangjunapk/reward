use sea_orm::prelude::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserFeeRequest {
    pub service_provider_id: i64,
    pub out_user_id: i32,
    pub fee: Decimal,
}
