use actix_web::{App, HttpServer};
use repositories::init_db;

type ControllerServiceProvider = controller::service_provider::save;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().await;
    let hs = HttpServer::new(|| {
        App::new()
            .service(controller::service_provider::save)
            .service(controller::service_provider::update)
            .service(controller::user::save)
            .service(controller::user_relation::save)
            .service(controller::user_relation::children)
            .service(controller::user_relation::parent)
            .service(controller::user_fee::save)
    });
    hs.bind("127.0.0.1:8088")?.run().await
}
