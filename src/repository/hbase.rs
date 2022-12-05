use std::collections::BTreeMap;

use crate::models::errors::OrderServiceError;
use crate::models::{orders::Order};
use crate::repository::hbase_connection::HbaseClient;
use crate::repository::hbase_utils::{create_order_builder_from_hbase_row, build_single_column_filter};
use hbase_thrift::hbase::TScan;

use super::hbase_utils::create_scan;


pub fn create_order_table(mut client: impl HbaseClient) -> Result<(), OrderServiceError> {
    match client.create_table(
        "orders",
        vec!["info".into(), "ids".into(), "addr".into(), "ol".into()],
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(OrderServiceError::from(e)),
    }
}

pub fn get_order_row(row_id: &str, mut client: impl HbaseClient) -> Result<Order, OrderServiceError> {
    let r = client.get_row(row_id)?;
    let row = match r.get(0) {
        Some(v) => v,
        None => return Err(OrderServiceError::RowNotFound(row_id.to_owned())),
    };
    match Order::build(create_order_builder_from_hbase_row(row)) {
        Some(v) => Ok(v),
        None => return Err(OrderServiceError::OrderBuildFailed()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        repository::{hbase_connection::MockHbaseClient, hbase_utils::{order_to_trowresult}},
    };
    use hbase_thrift::{
        hbase::{BatchMutation, Text, TScan, TRowResult, TCell},
        Attributes,
    };
    use mockall::predicate::eq;

    // #[test]
    // fn test_debugging() {
    //     const DB_IP: &str = "165.22.194.124:9090";
    //     let con = crate::repository::hbase_connection::HbaseConnection::connect(&DB_IP).unwrap();
    //     let r = get_order_row("2441910473035", con);
    //     let order = match r {
    //         Ok(r) => r,
    //         Err(e) => panic!("ERRROR"),
    //     };
    // }
}
