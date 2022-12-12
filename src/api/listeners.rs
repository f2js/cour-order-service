use super::utils::{env::{get_kafka_ip, get_db_ip}, get_unix_time};
use crate::{consumers::consumers::listen_for_events, models::{orders::{OrderEvent, OrderState}, errors::OrderServiceError}, repository::{hbase_connection::HbaseConnection, hbase}};

pub fn start_listener() {
    let kafka_ip = match get_kafka_ip() {
        Some(v) => v,
        None => return ,
    };

    let res = listen_for_events(
        |msg| {
            println!("I AM CALLED OUT FOR DELIVERY!");
            let db_ip = match get_db_ip() {
                Some(v) => v,
                None => return Err(OrderServiceError::SplitColumnError("TEMP ERROR FROM UTF8".into())),
            };
            println!("db_ip: {}", db_ip);
            let order = OrderEvent::from_bytes(msg.value)?;
            println!("order: {}", order.orderId);
            let con = HbaseConnection::connect(&db_ip)?;
            hbase::update_order_state(&order.orderId, OrderState::OutForDelivery, get_unix_time(), con)?;
            println!("Successfully updated the state of an order to OutForDelivery!");
            Ok(())
        },
        |msg| {
            println!("I AM CALLED DELIVERED!");
            let db_ip = match get_db_ip() {
                Some(v) => v,
                None => return Err(OrderServiceError::SplitColumnError("TEMP ERROR FROM UTF8".into())),
            };
            println!("db_ip: {}", db_ip);
            let order = OrderEvent::from_bytes(msg.value)?;
            println!("order: {}", order.orderId);
            let con = HbaseConnection::connect(&db_ip)?;
            hbase::update_order_state(&order.orderId, OrderState::Delivered, get_unix_time(), con)?;
            println!("Successfully updated the state of an order to Delivered!");
            Ok(())
        },
        &kafka_ip
    );
    println!("Listening ended due to error: {}", res.is_err());
}