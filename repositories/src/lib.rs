pub mod service_provider;
pub mod user;
pub mod user_relation;
pub mod user_fee;
mod user_reward;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::OnceLock;
use std::time::Duration;

pub static DB: OnceLock<DatabaseConnection> = OnceLock::new();



pub async fn init_db() {
    let mut opt = ConnectOptions::new("postgres://lenny:lenny@localhost/reward");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); //翻译不太一样，在dbeaver中被称为模式
    let db = Database::connect(opt).await.unwrap();
    DB.set(db).unwrap();
}
