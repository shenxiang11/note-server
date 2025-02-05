use anyhow::Result;
use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};
use std::sync::Arc;
use std::time::Duration;

pub struct MessageQueue {
    brokers: Vec<String>,
}

impl MessageQueue {
    pub fn new(brokers: Vec<String>) -> Arc<MessageQueue> {
        Arc::new(MessageQueue { brokers })
    }

    pub fn produce_message(&self, key: &[u8], data: &[u8], topic: &str) -> Result<()> {
        let mut producer = Producer::from_hosts(self.brokers.clone())
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()?;

        producer.send(&Record::from_key_value(topic, key, data))?;

        Ok(())
    }
}
