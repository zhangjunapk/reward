use crate::user;
use common::operate_receipt::OperateReceipt;
use entity::service_provider::Status::Disabled;
use entity::user_relation::{ActiveModel, Model};

type ServiceProviderRepositories = repositories::service_provider::ServiceProvider;

type UserRelationRepositories = repositories::user_relation::UserRelation;
type UserService = user::User;

pub struct UserRelation;

impl UserRelation {
    ///
    ///
    pub async fn save(user_relation: &mut ActiveModel) -> Result<(), OperateReceipt> {
        let service_provider = ServiceProviderRepositories::find_one_by_id(
            user_relation.clone().service_provider_id.unwrap(),
        )
        .await;

        if let Ok(service_provider) = service_provider {
            match service_provider {
                None => {
                    return Err(OperateReceipt::Error("服务商不存在".to_string()));
                }
                Some(service_provider) => {
                    if Disabled == service_provider.status {
                        return Err(OperateReceipt::Error("服务商已被禁用".to_string()));
                    }
                }
            }
        }

        let count = UserRelationRepositories::find_exist(
            user_relation.clone().service_provider_id.unwrap(),
            user_relation.clone().user_id.unwrap(),
        )
        .await;
        if let Ok(count) = count {
            if count > 0 {
                return Err(OperateReceipt::Error("已存在".to_string()));
            }
        }

        let parent = UserRelationRepositories::find_one_parent(
            user_relation.clone().service_provider_id.unwrap(),
            user_relation.clone().parent_id.unwrap(),
        )
        .await
        .map_err(|db_err| OperateReceipt::Error(db_err.to_string()))?;
        match parent {
            Some(parent) => {
                let parent_user = UserService::find_one(user_relation.clone().parent_id.unwrap())
                    .await
                    .unwrap();
                match parent_user {
                    Some(_) => {
                        &user_relation.left.set_if_not_equals(parent.left + 1);
                        &user_relation.right.set_if_not_equals(parent.right + 1);
                        &user_relation.level.set_if_not_equals(parent.level + 1);

                        let _ = UserRelationRepositories::parent_right_up(
                            user_relation.clone().left.unwrap(),
                            user_relation.clone().right.unwrap(),
                        )
                        .await;
                        let _ = UserRelationRepositories::insert(user_relation.clone()).await;

                        Ok(())
                    }
                    None => Err(OperateReceipt::Error("父用户不存在".to_string())),
                }
            }
            None => {
                if user_relation.clone().parent_id.unwrap().eq(&-1) {
                    let _ = UserRelationRepositories::insert(user_relation.clone()).await;
                    Ok(())
                } else {
                    Err(OperateReceipt::Error("找不到父节点".to_string()))
                }
            }
        }
    }

    pub async fn parent(
        service_provider_id: i64,
        out_user_id: i32,
    ) -> Result<Vec<Model>, OperateReceipt> {
        let user_relation =
            UserRelationRepositories::find_one_by_out(service_provider_id, out_user_id).await;
        match user_relation {
            Ok(user_relation) => {
                if let Some(user_relation) = user_relation {
                    Ok(UserRelationRepositories::parent(user_relation.left, user_relation.right)
                        .await.map_err(|db_err| OperateReceipt::Error(db_err.to_string()))?)
                }else{
                    Err(OperateReceipt::Error("找不到关系".to_string()))
                }
            }
            Err(err) => {
                Err(OperateReceipt::Error(err.to_string()))
            }
        }
    }

    pub async fn children( service_provider_id: i64,
                           out_user_id: i32,) -> Result<Vec<Model>, OperateReceipt> {
        let user_relation =
            UserRelationRepositories::find_one_by_out(service_provider_id, out_user_id).await;
        match user_relation {
            Ok(user_relation) => {
                if let Some(user_relation) = user_relation {
                    Ok(UserRelationRepositories::child(user_relation.left, user_relation.right)
                        .await.map_err(|db_err| OperateReceipt::Error(db_err.to_string()))?)
                }else{
                    Err(OperateReceipt::Error("找不到关系".to_string()))
                }
            }
            Err(err) => {
                Err(OperateReceipt::Error(err.to_string()))
            }
        }
    }
}
