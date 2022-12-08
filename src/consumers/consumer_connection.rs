use kafka::consumer::{Consumer, GroupOffsetStorage, FetchOffset, MessageSets, MessageSet, Message};
use crate::models::{orders::Order, errors::OrderServiceError};

#[cfg_attr(test, mockall::automock)]
pub trait KafkaConsumer {
    fn consume(&mut self, on_consumed: fn(&Message)->Result<(), OrderServiceError>);
}

pub struct KafkaConsConnection {
    con: Consumer
}

impl KafkaConsConnection {
    pub fn connect(topic: String, kafka_ip: String) -> Result<Self, OrderServiceError> { 
        let con = Consumer::from_hosts(vec!(kafka_ip))
            .with_topic(topic)
            .with_group("order".into())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()?;
        Ok(Self {
            con
        })
    }
}

impl KafkaConsumer for KafkaConsConnection {
    fn consume(&mut self, on_consumed: fn(&Message)->Result<(), OrderServiceError>) {
        for ms in self.con.poll().unwrap().iter() {
            println!("Found message");
            for m in ms.messages() {
              on_consumed(m);
            }
            self.con.consume_messageset(ms);
          }
          self.con.commit_consumed().unwrap();
    }
}