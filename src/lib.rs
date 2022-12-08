pub mod api;
pub mod models;
mod repository;
mod producers;
mod consumers;

use std::thread;

use actix_web::{App, HttpServer};

pub async fn run_api() -> std::io::Result<()>{
    thread::spawn(|| {
        api::listeners::start_listener();
    });
    HttpServer::new(|| {
        App::new()
            // register HTTP requests handlers
            .service(api::endpoints::index)
            .service(api::endpoints::get_order)
            .service(api::endpoints::pickup_order)
            .service(api::endpoints::deliver_order)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}