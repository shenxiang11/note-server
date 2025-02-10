use crate::model::{CountBiz, NoteReadMessage, UserHistoryBiz};
use crate::repository::InteractiveRepo;
use kafka::client::GroupOffsetStorage;
use kafka::consumer::Consumer;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, error};

pub struct NoteReadConsumer {
    brokers: Vec<String>,
    interactive_repo: InteractiveRepo,
}

impl NoteReadConsumer {
    pub fn new(brokers: Vec<String>, interactive_repo: InteractiveRepo) -> Self {
        Self {
            brokers,
            interactive_repo,
        }
    }

    pub fn consume(&self) -> anyhow::Result<()> {
        let mut con = Consumer::from_hosts(self.brokers.clone())
            .with_topic("NoteRead".to_string())
            .with_group("Note".to_string())
            .with_fallback_offset(kafka::client::FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()?;

        let mut messages_batch = Vec::new();

        loop {
            let mss = con.poll()?;

            for ms in mss.iter() {
                for m in ms.messages() {
                    let ret: serde_json::error::Result<NoteReadMessage> =
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
            self.handle_count(messages_batch.clone());
            self.handle_history(messages_batch.clone());
            messages_batch.clear();

            con.commit_consumed()?;
        }
    }

    fn handle_history(&self, messages: Vec<NoteReadMessage>) {
        if messages.is_empty() {
            return;
        }
        let messages: Vec<NoteReadMessage> = messages
            .into_iter()
            .filter(|message| message.user_id.is_some())
            .collect();
        if messages.is_empty() {
            return;
        }
        let interactive_repo = self.interactive_repo.clone();
        tokio::spawn(async move {
            let ret = interactive_repo
                .save_histories(UserHistoryBiz::Note, messages)
                .await;
            if let Err(e) = ret {
                error!("failed to save read history: {}", e);
            }
        });
    }

    fn handle_count(&self, messages: Vec<NoteReadMessage>) {
        if messages.is_empty() {
            return;
        }
        let mut count_map = std::collections::HashMap::new();
        for message in messages.iter() {
            let count = count_map.entry(message.biz_id).or_insert(0);
            *count += 1;
        }

        let interactive_repo = self.interactive_repo.clone();
        tokio::spawn(async move {
            for (biz_id, count) in count_map.iter() {
                println!("biz_id: {}, count: {}", biz_id, count);
                let ret = interactive_repo
                    .save_count(CountBiz::NoteRead, *biz_id, *count)
                    .await;
                if let Err(e) = ret {
                    error!("failed to add count: {}", e);
                }
            }
        });
    }
}
