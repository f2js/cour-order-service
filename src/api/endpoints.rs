use crate::{api::utils::env::{get_db_ip, get_kafka_ip, DB_IP_ENV_ERR_MSG, KAFKA_IP_ENV_ERR_MSG}};
use actix_web::{get, post, HttpResponse, Responder, web, HttpResponseBuilder};
use serde::Serialize;
use super::workers;
// const DB_IP: &str = "165.22.194.124:9090";

#[get("/")]
pub async fn index() -> String {
    "Service is running".to_string()
}

#[get("/order/{id}")]
pub async fn get_order(path: web::Path<String>) -> impl Responder {
    let db_ip = match get_db_ip() {
        Some(v) => v,
        None => return generate_response(&mut HttpResponse::InternalServerError(), DB_IP_ENV_ERR_MSG),
    };
    let id = path.into_inner();
    match workers::get_row(&id, &db_ip) {
        Ok(r) => 
            return generate_response(&mut HttpResponse::Ok(),format!("Successfully got order: {:?}", r.o_id)),
        Err(e) =>
            return generate_response(&mut HttpResponse::InternalServerError(), e.to_string()),
    }
}

// #[post("order/pickup/{id}")]
// pub async fn pickup_order(path: web::Path<String) -> impl Responder {
//     let db_ip = match get_db_ip() {
//         Some(v) => v,
//         None => return generate_err_response(&mut HttpResponse::InternalServerError(), DB_IP_ENV_ERR_MSG),
//     };
//     let id = path.into_inner();
//     return HttpResponse::Ok();
// }

fn generate_response(response_builder: &mut HttpResponseBuilder, error: impl Serialize) -> HttpResponse {
    response_builder.content_type("APPLICATION_JSON").json(error)
}