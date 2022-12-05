use actix_web::{web};

use crate::{models::{orders::Order, errors::OrderServiceError}, repository::{hbase_connection::HbaseConnection, hbase}, producers::{producers, producer_connection::KafkaProdConnection}};

pub fn get_row(row_id: &str, db_ip: &str) -> Result<Order, OrderServiceError> {
    let con = HbaseConnection::connect(db_ip)?;
    hbase::get_order_row(row_id, con)
}

pub fn create_table(db_ip: &str) -> Result<(), OrderServiceError> {
    let con = HbaseConnection::connect(db_ip)?;
    hbase::create_order_table(con)
}