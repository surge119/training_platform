extern crate core;


use std::sync::Mutex;

use actix_web::{App, HttpServer, Responder, ResponseError, web, web::Data};
use serde::{Deserialize, Serialize};

use container::Machine;
use docker::DockerController;

mod container;
mod docker;
mod logger;


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
    for network in main_frame.networks.clone() {
        // let result = network.1.labs.get(&info.name);
        // if result.is_some() {
        //     // If this call fails,the thread will panic. Can't error check with ?
        //     // operator b/c the function implements Future. Not a big deal for now,
        //     // we can't add semaphores b/c of this
        //     main_frame.docker_controller.start_docker_container(&info.name).await;
        //     return web::Json(true);
        // }
    }
    return web::Json(false);
}

async fn stop_box(info: web::Json<Box>, data: Data<container::Containers>) -> impl Responder {
    let main_frame = data;
    for network in main_frame.networks.clone() {
        // let result = network.1.labs.get(&info.name);
        // if result.is_some() {
        //     // If this call fails,the thread will panic. Can't error check with ?
        //     // operator b/c the function implements Future. Not a big deal for now,
        //     // we can't add semaphores b/c of this
        //     main_frame.docker_controller.stop_docker_container(&info.name).await;
        //     return web::Json(true);
        // }
    }
    return web::Json(false);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Struct that is shared between all parts of the app, can share
    // Container info and docker controller easily
    let data = Data::new(container::init_containers());

    println!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(
                web::scope("/api")
                    .route("/boxes", web::get().to(get_boxes))
                    .route("/status", web::get().to(check_server_health))
                    .route("/start_box", web::post().to(start_box))
                    .route("/stop_box", web::post().to(stop_box))
            )
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
