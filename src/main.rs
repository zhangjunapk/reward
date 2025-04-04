use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use repositories::init_db;

type ControllerServiceProvider = controller::service_provider::save;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().await;
    HttpServer::new(|| App::new().service(controller::service_provider::save))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
