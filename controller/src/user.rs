use common::request::user_request::UserRequest;
use actix_web::{post, web};
use common::global_response::GlobalResponse;
use common::response;

type UserService = service::user::User;

#[post("/user/save")]
pub async fn save(user: web::Json<UserRequest>) -> GlobalResponse<i64> {
    response!(UserService::save(user.0.into()).await)
}
