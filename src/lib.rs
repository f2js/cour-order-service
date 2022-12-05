pub mod api;
pub mod models;
mod repository;
mod producers;

use actix_web::{App, HttpServer};

pub async fn run_api() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            // register HTTP requests handlers
            .service(api::endpoints::index)
            .service(api::endpoints::get_order)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}