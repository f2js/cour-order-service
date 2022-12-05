#[cfg(test)]
mod integration_tests {
    extern crate cour_order_service;
    use std::collections::HashMap;

    use actix_web::web::Json;
    use testcontainers::{core::WaitFor, images::generic::GenericImage, *};

    use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

    
}
