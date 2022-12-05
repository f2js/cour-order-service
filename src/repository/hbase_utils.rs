use hbase_thrift::{hbase::{BatchMutation, TScan}, MutationBuilder, BatchMutationBuilder};

use rand::prelude::*;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;

use crate::models::{orders::{Order, OrderBuilder}};


pub fn create_order_builder_from_hbase_row(
    hbase_row: &hbase_thrift::hbase::TRowResult,
) -> OrderBuilder {
    let mut order_builder = OrderBuilder::default();
    let cols = match &hbase_row.columns{
        Some(v) =>v,
        None => return order_builder,
    };
    order_builder.o_id = get_value(hbase_row.row.clone());
    for (col, cell) in cols.iter() {
        let col = col.clone();
        let cell = cell.clone().value;
        let (column, value) = match get_column_and_value(&col, cell) {
            Some(v) => v,
            None => continue,
        };
        set_order_field(column, value, &mut order_builder);
    }
    order_builder
}


// fn get_column(col: &Vec<u8>) -> Option<(String, String)> {
//     let column: String = match std::str::from_utf8(col) {
//         Ok(colname) => colname.to_string(),
//         Err(_) => return None,
//     };
//     Some(match column.split_once(':') {
//         Some(v) => (v.0.to_owned(), v.1.to_owned()),
//         None => return None,
//     })
// }

fn get_column(col: &[u8]) -> Option<(String, String)> {
    let column = std::str::from_utf8(col).ok()?;
    let parts: Vec<&str> = column.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    Some((parts[0].to_owned(), parts[1].to_owned()))
}

fn get_value(cell: Option<Vec<u8>>) -> Option<String> {
    let cell = cell?;
    Some(match std::str::from_utf8(&cell) {
        Ok(val) => val.to_string(),
        Err(_) => return None,
    })
}

fn get_column_and_value(col: &Vec<u8>, cell: Option<Vec<u8>>) -> Option<((String, String),String)> {
    Some((get_column(&col)?, get_value(cell)?))
}

fn set_order_field(field: (String, String), val: String, order_builder: &mut OrderBuilder) {
    let col: (&str, &str) = (&field.0, &field.1);
    match col {
        ("info", "o_id") => order_builder.o_id = Some(val.clone()),
        ("ids", "c_id") => order_builder.c_id = Some(val.clone()),
        ("ids", "r_id") => order_builder.r_id = Some(val.clone()),
        ("addr", "c_addr") => order_builder.cust_addr = Some(val.clone()),
        ("addr", "r_addr") => order_builder.rest_addr = Some(val.clone()),
        (_, _) => println!("Unknown column type"),
    }
}

pub fn build_single_column_filter(colfam: &str, col: &str, operator: &str, value: &str) -> String {
    format!("SingleColumnValueFilter('{colfam}', '{col}', {operator}, 'binaryprefix:{value}')")
}

pub fn create_scan(columns_to_fetch: Vec<Vec<u8>>, filter_colfam: &str, filter_col: &str, filter_val: &str) -> TScan {
    TScan {
        columns: Some(columns_to_fetch),
        filter_string: Some(build_single_column_filter(filter_colfam, filter_col, "=", &filter_val).into()),
        start_row: None,
        stop_row: None,
        timestamp: None,
        caching: None,
        batch_size: Some(0),
        sort_columns: Some(false),
        reversed: Some(false),
        cache_blocks: Some(false),
    }
}

// Only for testing purposes 
pub(crate) fn order_to_trowresult(order: Order) -> hbase_thrift::hbase::TRowResult {
    let mut columns: std::collections::BTreeMap<hbase_thrift::hbase::Text, hbase_thrift::hbase::TCell> = std::collections::BTreeMap::new();
    columns.insert("ids:c_id".as_bytes().to_vec(), _to_tcell(&order.c_id));
    columns.insert("ids:r_id".as_bytes().to_vec(), _to_tcell(&order.r_id));
    columns.insert("addr:c_addr".as_bytes().to_vec(), _to_tcell(&order.cust_addr));
    columns.insert("addr:r_addr".as_bytes().to_vec(), _to_tcell(&order.rest_addr));
    hbase_thrift::hbase::TRowResult { row: Some(order.o_id.as_bytes().to_vec()), columns: Some(columns), sorted_columns: None }
}

fn _to_tcell(val: &str) -> hbase_thrift::hbase::TCell {
    hbase_thrift::hbase::TCell { value: Some(val.as_bytes().to_vec()), timestamp: Some(0) }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr};

    use super::*;
    use crate::models::orders::{Order, OrderBuilder};

    #[test]
    fn test_create_order_builder_from_hbase_row_unknown_field() {
        let order = Order{cust_addr: "addr".into(), rest_addr: "addr2".into(), c_id: "custid".into(), r_id: "restid".into(), o_id: "o_id".into() };
        let mut columns: std::collections::BTreeMap<hbase_thrift::hbase::Text, hbase_thrift::hbase::TCell> = std::collections::BTreeMap::new();
        columns.insert("ids:c_id".as_bytes().to_vec(), _to_tcell(&order.c_id));
        columns.insert("ids:r_id".as_bytes().to_vec(), _to_tcell(&order.r_id));
        columns.insert("addr:c_addr".as_bytes().to_vec(), _to_tcell(&order.cust_addr));
        columns.insert("addr:r_addr".as_bytes().to_vec(), _to_tcell(&order.rest_addr));
        let trowresult = hbase_thrift::hbase::TRowResult { row: Some(order.o_id.as_bytes().to_vec()), columns: Some(columns), sorted_columns: None };
        let obuilder = create_order_builder_from_hbase_row(&trowresult);
        
        assert!(obuilder.c_id.is_some());
        assert!(obuilder.r_id.is_some());
        assert!(obuilder.o_id.is_some());
        assert!(obuilder.cust_addr.is_some());
        assert!(obuilder.rest_addr.is_some());
    }

    #[test]
    fn test_create_order_builder_from_hbase_row_missing_field() {
        let order = Order{cust_addr: "addr".into(), rest_addr: "addr2".into(), c_id: "custid".into(), r_id: "restid".into(), o_id: "o_id".into() };
        let mut columns: std::collections::BTreeMap<hbase_thrift::hbase::Text, hbase_thrift::hbase::TCell> = std::collections::BTreeMap::new();
        columns.insert("info:o_id".as_bytes().to_vec(), _to_tcell(&order.o_id));
        // columns.insert("ids:c_id".as_bytes().to_vec(), _to_tcell(&order.c_id));
        columns.insert("ids:r_id".as_bytes().to_vec(), _to_tcell(&order.r_id));
        columns.insert("addr:c_addr".as_bytes().to_vec(), _to_tcell(&order.cust_addr));
        columns.insert("addr:r_addr".as_bytes().to_vec(), _to_tcell(&order.rest_addr));
        let trowresult = hbase_thrift::hbase::TRowResult { row: Some(order.o_id.as_bytes().to_vec()), columns: Some(columns), sorted_columns: None };
        let obuilder = create_order_builder_from_hbase_row(&trowresult);
        assert!(obuilder.c_id.is_none());

        assert!(obuilder.o_id.is_some());
        assert!(obuilder.r_id.is_some());
        assert!(obuilder.cust_addr.is_some());
        assert!(obuilder.rest_addr.is_some());
    }

    #[test]
    fn test_create_order_builder_from_hbase_row_on_content() {
        let order = Order{cust_addr: "addr".into(), rest_addr: "addr2".into(), c_id: "custid".into(), r_id: "restid".into(), o_id: "o_id".into() };
        let trowresult = order_to_trowresult(order.clone());
        let obuilder = create_order_builder_from_hbase_row(&trowresult);
        assert_eq!(obuilder.o_id.unwrap(), order.o_id);
        assert_eq!(obuilder.r_id.unwrap(), order.r_id);
        assert_eq!(obuilder.c_id.unwrap(), order.c_id);
        assert_eq!(obuilder.cust_addr.unwrap(), order.cust_addr);
        assert_eq!(obuilder.rest_addr.unwrap(), order.rest_addr);
    }

    #[test]
    fn test_create_order_builder_from_hbase_row_on_content_empty_order() {
        let order = Order{cust_addr: "addr".into(), rest_addr: "addr2".into(), c_id: "custid".into(), r_id: "restid".into(), o_id: "o_id".into() };
        let trowresult = order_to_trowresult(order.clone());
        let obuilder = create_order_builder_from_hbase_row(&trowresult);
        assert_eq!(obuilder.o_id.unwrap(), order.o_id);
        assert_eq!(obuilder.r_id.unwrap(), order.r_id);
        assert_eq!(obuilder.c_id.unwrap(), order.c_id);
        assert_eq!(obuilder.cust_addr.unwrap(), order.cust_addr);
        assert_eq!(obuilder.rest_addr.unwrap(), order.rest_addr);
    }

    #[test]
    fn test_create_order_builder_from_hbase_row_is_some() {
        let order = Order{cust_addr: "addr".into(), rest_addr: "addr2".into(), c_id: "custid".into(), r_id: "restid".into(), o_id: "o_id".into() };
        let trowresult = order_to_trowresult(order);
        let obuilder = create_order_builder_from_hbase_row(&trowresult);
        assert!(obuilder.o_id.is_some());
        assert!(obuilder.c_id.is_some());
        assert!(obuilder.r_id.is_some());
        assert!(obuilder.cust_addr.is_some());
        assert!(obuilder.rest_addr.is_some());
    }

    #[test]
    fn test_create_scan_with_cols() {
        let cols = vec!["col1:col".into(), "col2".into()];
        let colfam = "testcolfam";
        let col = "testcol";
        let val = "testval";
        let scan = create_scan(cols.clone(), colfam, col, val);
        assert_eq!(scan.columns.unwrap(), cols);
        assert_eq!(scan.filter_string.unwrap(), Into::<Vec<u8>>::into(build_single_column_filter(colfam, col, "=", val)));
    }

    #[test]
    fn test_create_scan_no_cols() {
        let cols = vec![];
        let colfam = "testcolfam";
        let col = "testcol";
        let val = "testval";
        let scan = create_scan(cols.clone(), colfam, col, val);
        assert_eq!(scan.columns.unwrap(), cols);
        assert_eq!(scan.filter_string.unwrap(), Into::<Vec<u8>>::into(build_single_column_filter(colfam, col, "=", val)));
    }

    #[test]
    fn test_build_single_column_filter() {
        let exp = "SingleColumnValueFilter('test', 'test', =, 'binaryprefix:test')";
        let actual = build_single_column_filter("test", "test", "=", "test");
        assert_eq!(actual, exp.to_string());
    }

    #[test]
    fn test_get_column_bad_str() {
        let input:Vec<u8> = vec![255,255,58,255,255];
        let actual = get_column(&input);
        assert!(actual.is_none());
    }

    #[test]
    fn test_get_column_bad_split() {
        let input:Vec<u8> = "colfamcol".into();
        let actual = get_column(&input);
        assert!(actual.is_none());
    }

    #[test]
    fn test_get_column() {
        let expected = ("colfam", "col");
        let input:Vec<u8> = "colfam:col".into();
        let actual = get_column(&input);
        assert!(actual.is_some());
        let actual = actual.unwrap();
        assert_eq!(actual.0, expected.0);
        assert_eq!(actual.1, expected.1);
    }

    #[test]
    fn test_get_value_none() {
        let actual = get_value(None);
        assert!(actual.is_none());
    }

    #[test]
    fn test_get_value_bad_str() {
        let input: Vec<u8> = vec![255,255,255,255];
        let actual = get_value(Some(input));
        assert!(actual.is_none());
    }

    #[test]
    fn test_get_value() {
        let expected = "hello, world";
        let input: Vec<u8> = expected.into();
        let actual = get_value(Some(input));
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_set_order_field_bad_col() {
        let field = ("addr".to_string(), "o_id".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.o_id.is_none());
        assert!(order_builder.r_id.is_none());
        assert!(order_builder.c_id.is_none());
        assert!(order_builder.rest_addr.is_none());
        assert!(order_builder.cust_addr.is_none());
    }

    #[test]
    fn test_set_order_field_bad_colfam() {
        let field = ("aaaaadr".to_string(), "r_addr".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.o_id.is_none());
        assert!(order_builder.r_id.is_none());
        assert!(order_builder.c_id.is_none());
        assert!(order_builder.rest_addr.is_none());
        assert!(order_builder.cust_addr.is_none());
    }

    #[test]
    fn test_set_order_field_r_addr() {
        let field = ("addr".to_string(), "r_addr".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.rest_addr.is_some());
        assert_eq!(order_builder.rest_addr.unwrap(), val);
    }

    #[test]
    fn test_set_order_field_c_addr() {
        let field = ("addr".to_string(), "c_addr".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.cust_addr.is_some());
        assert_eq!(order_builder.cust_addr.unwrap(), val);
    }

    #[test]
    fn test_set_order_field_r_id() {
        let field = ("ids".to_string(), "r_id".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.r_id.is_some());
        assert_eq!(order_builder.r_id.unwrap(), val);
    }

    #[test]
    fn test_set_order_field_c_id() {
        let field = ("ids".to_string(), "c_id".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.c_id.is_some());
        assert_eq!(order_builder.c_id.unwrap(), val);
    }

    #[test]
    fn test_set_order_field_o_id() {
        let field = ("info".to_string(), "o_id".to_string());
        let val = "value".to_string();
        let mut order_builder = OrderBuilder::default();
        set_order_field(field, val.clone(), &mut order_builder);
        assert!(order_builder.o_id.is_some());
        assert_eq!(order_builder.o_id.unwrap(), val);
    }

    fn tuple_to_u8_vec(tuple: (&str, &str)) -> Vec<u8> {
        format!("{}:{}", tuple.0, tuple.1).into()
    }
}