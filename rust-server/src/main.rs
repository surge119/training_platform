extern crate core;

use actix_web::{web, web::Data, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod config;
mod container;
mod docker;

#[derive(Serialize)]
struct Status {
    boxes: bool,
    used_instances: u8,
    available_instances: u8,
}

async fn get_boxes(data: Data<container::Containers>) -> impl Responder {
    return web::Json(data.networks.clone());
}

async fn check_server_health() -> impl Responder {
    return web::Json(Status {
        boxes: true,
        used_instances: 0,
        available_instances: 0,
    });
}

#[derive(Deserialize, Serialize)]
struct Box {
    name: String,
}

//Start docker container
async fn start_box(info: web::Json<Box>, data: Data<container::Containers>) -> impl Responder {
    let main_frame = data;
    let result = main_frame
        .docker_controller
        .start_docker_container(&info.name)
        .await;
    println!("{:?}", result);
    if result.is_err() {
        return web::Json(false);
    }
    return web::Json(true);
}

async fn stop_box(info: web::Json<Box>, data: Data<container::Containers>) -> impl Responder {
    let main_frame = data;
    let result = main_frame
        .docker_controller
        .stop_docker_container(&info.name)
        .await;
    println!("{:?}", result);
    if result.is_err() {
        return web::Json(false);
    }
    return web::Json(true);
}

async fn reset_box(info: web::Json<Box>, data: Data<container::Containers>) -> impl Responder {
    let main_frame = data;
    let result = main_frame
        .docker_controller
        .reset_docker_container(&info.name)
        .await;
    println!("{:?}", result);
    if result.is_err() {
        return web::Json(false);
    }
    return web::Json(true);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Struct that is shared between all parts of the app, can share
    // Container info and docker controller easily
    let data = Data::new(container::init_containers());

    println!("{:?}", data);

    HttpServer::new(move || {
        App::new().app_data(Data::clone(&data)).service(
            web::scope("/api")
                .route("/boxes", web::get().to(get_boxes))
                .route("/status", web::get().to(check_server_health))
                .route("/start_box", web::post().to(start_box))
                .route("/stop_box", web::post().to(stop_box))
                .route("/reset_box", web::post().to(reset_box)),
        )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
