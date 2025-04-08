use actix_web::{post, web};
use common::global_response::GlobalResponse;
use common::request::user_fee_request::UserFeeRequest;
use common::response;
use service::user_fee;

type UserFeeService = user_fee::UserFee;

#[post("/user_fee/save")]
pub async fn save(user_fee: web::Json<UserFeeRequest>) -> GlobalResponse<()> {
    response!(UserFeeService::save(user_fee.0).await)
}

