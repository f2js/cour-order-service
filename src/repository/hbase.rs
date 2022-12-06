use std::collections::BTreeMap;

use crate::models::errors::OrderServiceError;
use crate::models::orders::OrderState;
use crate::models::{orders::Order};
use crate::repository::hbase_connection::HbaseClient;
use crate::repository::hbase_utils::{create_order_builder_from_hbase_row, build_single_column_filter};
use hbase_thrift::BatchMutationBuilder;
use hbase_thrift::hbase::TScan;

use super::hbase_utils::{create_scan, create_cell_mutation};


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

pub fn update_order_state(row_id: &str, new_order_state: OrderState, unix_time: i64, mut client: impl HbaseClient) -> Result<(), OrderServiceError>{
    let mutations = vec![create_cell_mutation("info", "state", new_order_state.to_string())];
    let batch = <BatchMutationBuilder>::default().row(row_id.clone()).mutations(mutations).build();
    client.put("orders", vec![batch], Some(unix_time), None)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        repository::{hbase_connection::MockHbaseClient, hbase_utils::{order_to_trowresult, _to_tcell}},
    };
    use hbase_thrift::{
        hbase::{BatchMutation, Text, TScan, TRowResult, TCell},
        Attributes,
    };
    use mockall::predicate::eq;

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    #[test]
    fn test_update_order_state_is_ok() {
        let userid = "id";
        let order_state = OrderState::OutForDelivery;
        let time: i64 = 10;
        let mut mock_con = MockHbaseClient::new();
        mock_con.expect_put()
            .withf(move |x, _y, z, æ| {
                x.eq("orders") && z.eq(&Some(time.clone())) && æ.is_none()
            })
            .times(1)
            .returning(|_x, _y, _z, _æ| {
                Err(thrift::Error::User("Error".into()))
            }
        );
        let res = update_order_state(userid, order_state, time, mock_con);
        assert!(res.is_err());
        let result_error = res.err().unwrap();
        assert_err!(result_error, OrderServiceError::DBError(_));
    }

    #[test]
    fn test_update_order_state_is_err() {
        let userid = "id";
        let order_state = OrderState::OutForDelivery;
        let time: i64 = 10;
        let mut mock_con = MockHbaseClient::new();
        mock_con.expect_put()
            .withf(move |x, _y, z, æ| {
                x.eq("orders") && z.eq(&Some(time.clone())) && æ.is_none()
            })
            .times(1)
            .returning(|_x, _y, _z, _æ| {
                Ok(())
            }
        );
        let res = update_order_state(userid, order_state, time, mock_con);
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_order_row_is_ok() {
        let userid = "id";
        let mut mock_con = MockHbaseClient::new();
        mock_con.expect_get_row()
            .with(eq(userid.clone()))
            .times(1)
            .returning(|x| {
                Ok(vec![order_to_trowresult(
                    Order {
                        o_id: x.clone().to_owned(),
                        c_id: "cust_id".to_owned(),
                        r_id: "rest_id".to_owned(),
                        cust_addr: "custaddr".to_owned(),
                        rest_addr: "restaddr".to_owned(),
                        state: "pending".to_owned(),
                    }
                )])
            });
        let res = get_order_row(userid.into(), mock_con);
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_order_row_success() {
        let userid = "id";
        let mut mock_con = MockHbaseClient::new();
        mock_con.expect_get_row()
            .with(eq(userid.clone()))
            .times(1)
            .returning(|x| {
                Ok(vec![order_to_trowresult(
                    Order {
                        o_id: x.clone().to_owned(),
                        c_id: "cust_id".to_owned(),
                        r_id: "rest_id".to_owned(),
                        cust_addr: "custaddr".to_owned(),
                        rest_addr: "restaddr".to_owned(),
                        state: "pending".to_owned(),
                    }
                )])
            });
        let res = get_order_row(userid.into(), mock_con).unwrap();
        assert_eq!(res.o_id, userid);
    }
    #[test]
    fn test_get_order_row_bad_trow_result() {
        let userid = "id";
        let mut mock_con = MockHbaseClient::new();
        mock_con.expect_get_row()
            .with(eq(userid.clone()))
            .times(1)
            .returning(|x| {
                let mut columns: std::collections::BTreeMap<hbase_thrift::hbase::Text, hbase_thrift::hbase::TCell> = std::collections::BTreeMap::new();
                columns.insert("ids:c_id".as_bytes().to_vec(), _to_tcell("cust_id"));
                columns.insert("BADCOLUMNFAMILYNAME:r_id".as_bytes().to_vec(), _to_tcell("rest_id"));
                columns.insert("addr:c_addr".as_bytes().to_vec(), _to_tcell("&order.cust_addr"));
                columns.insert("addr:r_addr".as_bytes().to_vec(), _to_tcell("&order.rest_addr"));
                let res = hbase_thrift::hbase::TRowResult { row: Some(x.as_bytes().to_vec()), columns: Some(columns), sorted_columns: None };
                Ok(vec![res])
            });
        let res = get_order_row(userid.into(), mock_con);
        assert!(res.is_err());
        let result_error = res.err().unwrap();
        assert_err!(result_error, OrderServiceError::OrderBuildFailed());
    }

    #[test]
    fn test_get_order_row_err() {
        let userid = "id";
        let mut mock_con = MockHbaseClient::new();
        mock_con.expect_get_row()
            .with(eq(userid.clone()))
            .times(1)
            .returning(move|_x| {
                Err(OrderServiceError::DBError(thrift::Error::User("Error".into())))
            });
        let res = get_order_row(userid.into(), mock_con);
        assert!(res.is_err());
        let result_error = res.err().unwrap();
        assert_err!(result_error, OrderServiceError::DBError(_));
    }
}
