use common::request::relation_user_request::RelationUserRequest;
use common::request::user_relation_request::UserRelationRequest;
use actix_web::{get, post, web};
use common::global_response::GlobalResponse;
use common::response;
use entity::user_relation::{ActiveModel, Model};

type UserRelationService = service::user_relation::UserRelation;

#[post("/user_relation/save")]
pub async fn save(user_relation: web::Json<UserRelationRequest>) -> GlobalResponse<()> {
    let user_relation: ActiveModel = user_relation.0.into();
    let mut user_relation = user_relation;
    response!(UserRelationService::save(&mut user_relation).await)
}

#[get("/user_relation/parent")]
pub async fn parent(user_id_param: web::Query<RelationUserRequest>) -> GlobalResponse<Vec<Model>> {
    response!(
        UserRelationService::parent(user_id_param.0.service_provider_id, user_id_param.0.out_user_id).await
    )
}

#[get("/user_relation/children")]
pub async fn children(user_id_param: web::Query<RelationUserRequest>) -> GlobalResponse<Vec<Model>> {
    response!(
        UserRelationService::children(user_id_param.0.service_provider_id, user_id_param.0.out_user_id).await
    )
}
