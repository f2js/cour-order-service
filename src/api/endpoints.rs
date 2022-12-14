use crate::{api::utils::{env::{get_db_ip, get_kafka_ip, DB_IP_ENV_ERR_MSG, KAFKA_IP_ENV_ERR_MSG}, generate_response}, models::errors::OrderServiceError};
use actix_web::{get, post, HttpResponse, Responder, web, HttpResponseBuilder};
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
            match e {
                OrderServiceError::RowNotFound(r) => return generate_response(&mut HttpResponse::NotFound(), format!("Order by id {} was not found.", r)),
                _ => return generate_response(&mut HttpResponse::InternalServerError(), e.to_string())
            }
    }
}

// #[post("order/pickup/{id}")]
// pub async fn pickup_order(path: web::Path<String>) -> impl Responder {
//     let db_ip = match get_db_ip() {
//         Some(v) => v,
//         None => return generate_response(&mut HttpResponse::InternalServerError(), DB_IP_ENV_ERR_MSG),
//     };
//     let kafka_ip = match get_kafka_ip() {
//         Some(v) => v,
//         None => return generate_response(&mut HttpResponse::InternalServerError(), KAFKA_IP_ENV_ERR_MSG),
//     };
//     let id = path.into_inner();
//     match workers::mark_order_as_out_for_delivery(&id, &db_ip, &kafka_ip) {
//         Ok(_) => 
//             return generate_response(&mut HttpResponse::Ok(),format!("Order is now out for delivery!")),
//         Err(e) =>
//             match e {
//                 OrderServiceError::RowNotFound(r) => return generate_response(&mut HttpResponse::NotFound(), format!("Order by id {} was not found.", r)),
//                 _ => return generate_response(&mut HttpResponse::InternalServerError(), e.to_string())
//             }
//     }
// }

// #[post("order/deliver/{id}")]
// pub async fn deliver_order(path: web::Path<String>) -> impl Responder {
//     let db_ip = match get_db_ip() {
//         Some(v) => v,
//         None => return generate_response(&mut HttpResponse::InternalServerError(), DB_IP_ENV_ERR_MSG),
//     };
//     let kafka_ip = match get_kafka_ip() {
//         Some(v) => v,
//         None => return generate_response(&mut HttpResponse::InternalServerError(), KAFKA_IP_ENV_ERR_MSG),
//     };
//     let id = path.into_inner();
//     match workers::mark_order_as_delivered(&id, &db_ip, &kafka_ip) {
//         Ok(_) => 
//             return generate_response(&mut HttpResponse::Ok(),format!("Order is now delivered!")),
//         Err(e) =>
//             match e {
//                 OrderServiceError::RowNotFound(r) => return generate_response(&mut HttpResponse::NotFound(), format!("Order by id {} was not found.", r)),
//                 _ => return generate_response(&mut HttpResponse::InternalServerError(), e.to_string())
//             }
//     }
// }