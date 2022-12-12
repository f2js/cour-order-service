use crate::models::{errors::OrderServiceError, orders::{ OrderEvent}};

use super::producer_connection::{KafkaProducer};

pub fn publish_order_out_for_delivery(order: OrderEvent, producer: &mut impl KafkaProducer) -> Result<(), OrderServiceError> {
    let json = order.to_json_string()?;
    producer.send("OrderOutForDelivery", json)
}

pub fn publish_order_delivered(order: OrderEvent, producer: &mut impl KafkaProducer) -> Result<(), OrderServiceError> {
    let json = order.to_json_string()?;
    producer.send("OrderDelivered", json)
}

#[cfg(test)]
mod tests {
    use crate::producers::producer_connection::MockKafkaProducer;

    use super::*;

    #[test]
    fn test_raise_event_out_for_delivery_is_ok() {
        let order = OrderEvent{orderId: "o_id".into(), courierId: "cour_id".into()};
        let exp_json  = format!(
            "{{\"orderId\":\"{}\",\"courierId\":\"{}\"}}", 
            order.orderId, order.courierId);
        let mut mock_prod = MockKafkaProducer::new();
        mock_prod.expect_send()
            .withf(move |x, y| {
                println!("{y}");
                x.eq("OrderOutForDelivery") && y.eq(&exp_json)
            })
            .times(1)
            .returning(|_x, _y| {
                Ok(())
            });
        let res = publish_order_out_for_delivery(order, &mut mock_prod);
        assert!(res.is_ok());
    }

    #[test]
    fn test_raise_event_out_for_delivery_is_err() {
        let order = OrderEvent{orderId: "o_id".into(), courierId: "cour_id".into()};
        let exp_json  = format!(
            "{{\"orderId\":\"{}\",\"courierId\":\"{}\"}}", 
            order.orderId, order.courierId);
        let mut mock_prod = MockKafkaProducer::new();
        mock_prod.expect_send()
            .withf(move |x, y| {
                x.eq("OrderOutForDelivery") && y.eq(&exp_json)
            })
            .times(1)
            .returning(|_x, _y| {
                Err(OrderServiceError::EventBrokerError(kafka::Error::CodecError))
            });
        let res = publish_order_out_for_delivery(order, &mut mock_prod);
        assert!(res.is_err());
    }

    #[test]
    fn test_raise_event_delivered_is_ok() {
        let order = OrderEvent{orderId: "o_id".into(), courierId: "cour_id".into()};
        let exp_json  = format!(
            "{{\"orderId\":\"{}\",\"courierId\":\"{}\"}}", 
            order.orderId, order.courierId);
        let mut mock_prod = MockKafkaProducer::new();
        mock_prod.expect_send()
            .withf(move |x, y| {
                println!("{y}");
                x.eq("OrderDelivered") && y.eq(&exp_json)
            })
            .times(1)
            .returning(|_x, _y| {
                Ok(())
            });
        let res = publish_order_delivered(order, &mut mock_prod);
        assert!(res.is_ok());
    }

    #[test]
    fn test_raise_event_delivered_is_err() {
        let order = OrderEvent{orderId: "o_id".into(), courierId: "cour_id".into()};
        let exp_json  = format!(
            "{{\"orderId\":\"{}\",\"courierId\":\"{}\"}}", 
            order.orderId, order.courierId);
        let mut mock_prod = MockKafkaProducer::new();
        mock_prod.expect_send()
            .withf(move |x, y| {
                x.eq("OrderDelivered") && y.eq(&exp_json)
            })
            .times(1)
            .returning(|_x, _y| {
                Err(OrderServiceError::EventBrokerError(kafka::Error::CodecError))
            });
        let res = publish_order_delivered(order, &mut mock_prod);
        assert!(res.is_err());
    }
}