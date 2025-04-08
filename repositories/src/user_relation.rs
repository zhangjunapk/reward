use crate::DB;
use entity::user_relation;
use entity::user_relation::{ActiveModel, Entity, Model};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, DbErr, EntityTrait, ExecResult, InsertResult,
    PaginatorTrait, QueryFilter, Statement, Value, Values,
};

pub struct UserRelation;

impl UserRelation {
    pub async fn parent_right_up(child_left: i32, child_right: i32) -> Result<ExecResult, DbErr> {
        DB.get()
            .unwrap()
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                format!("update user_relation set \"right\"=\"right\"+2 where \"left\"<{child_left} and \"right\"<={child_right}"),
            ))
            .await
    }

    pub async fn insert(user_relation: ActiveModel) -> Result<InsertResult<ActiveModel>, DbErr> {
        Entity::insert(user_relation).exec(DB.get().unwrap()).await
    }

    ///
    /// left <self.left && right > self.right
    ///
    pub async fn parent(left: i32, right: i32) -> Result<Vec<Model>, DbErr> {
        let result = Entity::find().from_raw_sql(Statement {
            sql: format!(
                "select * from user_relation where \"left\" <{left} and \"right\" >{right}"
            ),
            values: Some(Values(vec![Value::String(Some(Box::new("".to_string())))])),
            db_backend: DatabaseBackend::Postgres,
        });
        result.all(DB.get().unwrap()).await
    }

    ///
    /// left >self.left && right < self.right
    ///
    pub async fn child(left: i32, right: i32) -> Result<Vec<Model>, DbErr> {
        let result = Entity::find().from_raw_sql(Statement {
            sql: format!(
                "select * from user_relation where \"left\" >{left} and \"right\" <{right}"
            ),
            values: Some(Values(vec![Value::String(Some(Box::new("".to_string())))])),
            db_backend: DatabaseBackend::Postgres,
        });
        result.all(DB.get().unwrap()).await
    }

    pub async fn find_one(id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(DB.get().unwrap()).await
    }

    pub async fn find_one_by_out(
        service_provider_id: i64,
        out_user_id: i32,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find().from_raw_sql(
            Statement {
                sql: format!("select user_relation.* from user_relation ,\"user\"  where  \"user\".service_provider_id={service_provider_id}
and \"user\".out_user_id={out_user_id}
and user_relation.service_provider_id =\"user\".service_provider_id
and user_relation.user_id =\"user\".id"),
                values: Some(Values(vec![Value::BigInt(Some(service_provider_id.clone())), Value::Int(Some(out_user_id.clone()))])),
                db_backend: DatabaseBackend::Postgres,
            }
        ).one(DB.get().unwrap())
            .await
    }

    pub async fn find_one_parent(
        service_provider_id: i64,
        user_id: i64,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(
                user_relation::Column::ServiceProviderId
                    .eq(service_provider_id)
                    .and(user_relation::Column::UserId.eq(user_id)),
            )
            .one(DB.get().unwrap())
            .await
    }

    pub async fn find_exist(service_provider_id: i64, user_id: i64) -> Result<u64, DbErr> {
        Entity::find()
            .filter(
                user_relation::Column::ServiceProviderId
                    .eq(service_provider_id)
                    .and(user_relation::Column::UserId.eq(user_id)),
            )
            .count(DB.get().unwrap())
            .await
    }
}
