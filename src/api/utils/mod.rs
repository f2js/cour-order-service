use actix_web::{HttpResponseBuilder, HttpResponse};
use serde::Serialize;

pub mod env;

pub fn generate_response(response_builder: &mut HttpResponseBuilder, error: impl Serialize) -> HttpResponse {
    response_builder.content_type("APPLICATION_JSON").json(error)
}

pub fn get_unix_time() -> i64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64
}