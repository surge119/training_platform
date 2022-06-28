use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Box {
    name: String,
    instances: u32
}
#[derive(Serialize)]
struct Status {
    boxes: bool,
    used_instances: u8,
    available_instances: u8
}
async fn get_boxes()  -> impl Responder {
    //This will do some actual checking of boxes and their respective status
    return web::Json(Box{name:String::from("test"),instances:1});
}

async fn check_server_health() -> impl Responder{
    return web::Json(Status{boxes:true,used_instances: 0, available_instances: 0});
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/boxes", web::get().to(get_boxes))
                .route("/status", web::get().to(check_server_health))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}