use crate::model::{CountBiz, NoteLikeMessage, NoteReadMessage, UserHistoryBiz};
use crate::repository::InteractiveRepo;
use kafka::client::GroupOffsetStorage;
use kafka::consumer::Consumer;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, error};

pub struct NoteLikeConsumer {
    brokers: Vec<String>,
    interactive_repo: InteractiveRepo,
}

impl NoteLikeConsumer {
    pub fn new(brokers: Vec<String>, interactive_repo: InteractiveRepo) -> Self {
        Self {
            brokers,
            interactive_repo,
        }
    }

    pub fn consume(&self) -> anyhow::Result<()> {
        let mut con = Consumer::from_hosts(self.brokers.clone())
            .with_topic("NoteLike".to_string())
            .with_group("Note".to_string())
            .with_fallback_offset(kafka::client::FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()?;

        let mut messages_batch = Vec::new();

        loop {
            let mss = con.poll()?;

            for ms in mss.iter() {
                for m in ms.messages() {
                    let ret: serde_json::error::Result<NoteLikeMessage> =
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

            let interactive_repo = self.interactive_repo.clone();
            let messages_batch_cloned = messages_batch.clone();
            tokio::spawn(async move {
                handle_count(interactive_repo, messages_batch_cloned).await;
            });

            messages_batch.clear();

            con.commit_consumed()?;
        }
    }
}

async fn handle_count(interactive_repo: InteractiveRepo, messages: Vec<NoteLikeMessage>) {
    if messages.is_empty() {
        return;
    }
    let mut count_map = std::collections::HashMap::new();
    for message in messages.iter() {
        let count = count_map.entry(message.biz_id).or_insert(0);
        *count += if message.like { 1 } else { -1 };
    }

    let interactive_repo = interactive_repo.clone();
    for (biz_id, count) in count_map.iter() {
        let ret = interactive_repo
            .save_count(CountBiz::NoteLike, *biz_id, *count)
            .await;
        if let Err(e) = ret {
            error!("failed to add count: {}", e);
        }
    }
}
