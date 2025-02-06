use crate::repository::IMRepo;
use kafka::client::GroupOffsetStorage;
use kafka::consumer::Consumer;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, error};

pub struct UserRegisterConsumer {
    brokers: Vec<String>,
    im_repo: IMRepo,
}

impl UserRegisterConsumer {
    pub fn new(brokers: Vec<String>, im_repo: IMRepo) -> Self {
        Self { brokers, im_repo }
    }

    pub async fn consume(&self) -> anyhow::Result<()> {
        let mut con = Consumer::from_hosts(self.brokers.clone())
            .with_topic("UserRegister".to_string())
            .with_group("im".to_string())
            .with_fallback_offset(kafka::client::FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()?;

        let mut messages_batch = Vec::new();

        loop {
            let mss = con.poll()?;

            for ms in mss.iter() {
                for m in ms.messages() {
                    let ret: serde_json::error::Result<UserRegisterMessage> =
                        serde_json::from_slice(&m.value);

                    match ret {
                        Ok(data) => {
                            messages_batch.push(data);
                        }
                        Err(e) => {
                            error!("failed to deserialize note read message: {}", e);
                        }
                    }
                }
                con.consume_messageset(ms)?;
            }

            if !messages_batch.is_empty() {
                self.im_repo.create_users(messages_batch.clone()).await?;
            }

            messages_batch.clear();

            con.commit_consumed()?;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisterMessage {
    #[serde(rename = "userID")]
    pub user_id: i64,
    pub nickname: String,
    #[serde(rename = "faceURL")]
    pub face_url: String,
}
