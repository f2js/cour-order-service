use kafka::consumer::Message;
use serde::Deserialize;

use crate::models::{errors::OrderServiceError, orders::OrderEvent};

use super::consumer_connection::{KafkaConsumer, KafkaConsConnection};

pub fn listen_for_events(
    on_picked_up: fn(&Message)->Result<(), OrderServiceError>, 
    on_delivered: fn(&Message)->Result<(), OrderServiceError>, 
    kafka_ip: &str
) -> Result<(), OrderServiceError>{
    let mut picked_up_consumer = KafkaConsConnection::connect("OrderOutForDelivery".into(), kafka_ip.into())?;
    let mut delivered_consumer = KafkaConsConnection::connect("OrderDelivered".into(), kafka_ip.into())?;

    loop {
        picked_up_consumer.consume(on_picked_up);
        delivered_consumer.consume(on_delivered);
    }
}

