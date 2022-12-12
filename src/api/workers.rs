use actix_web::{web};

use crate::{models::{orders::{Order, OrderState, OrderEvent}, errors::OrderServiceError}, 
repository::{hbase_connection::HbaseConnection, hbase}, 
producers::{producers, producer_connection::KafkaProdConnection},
api::utils::get_unix_time};

pub fn get_row(row_id: &str, db_ip: &str) -> Result<Order, OrderServiceError> {
    let con = HbaseConnection::connect(db_ip)?;
    hbase::get_order_row(row_id, con)
}

pub fn create_table(db_ip: &str) -> Result<(), OrderServiceError> {
    let con = HbaseConnection::connect(db_ip)?;
    hbase::create_order_table(con)
}

// pub fn mark_order_as_out_for_delivery(row_id: &str, db_ip: &str, kafka_ip: &str) -> Result<(), OrderServiceError> {
//     // let con = HbaseConnection::connect(db_ip)?;
//     // hbase::update_order_state(row_id, OrderState::OutForDelivery, get_unix_time(), con)?;

//     let mut kafka_con = KafkaProdConnection::connect(kafka_ip.into())?;
//     producers::publish_order_out_for_delivery(OrderEvent{orderId: row_id.to_owned()}, &mut kafka_con)?;
//     Ok(())
// }

// pub fn mark_order_as_delivered(row_id: &str, db_ip: &str, kafka_ip: &str) -> Result<(), OrderServiceError> {
//     // let con = HbaseConnection::connect(db_ip)?;
//     // hbase::update_order_state(row_id.clone(), OrderState::Delivered, get_unix_time(), con)?;

//     let mut kafka_con = KafkaProdConnection::connect(kafka_ip.into())?;
//     producers::publish_order_delivered(OrderEvent{orderId: row_id.to_owned()}, &mut kafka_con)?;
//     Ok(())
// }