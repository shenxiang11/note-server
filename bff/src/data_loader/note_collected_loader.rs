use crate::service::interactive::InteractiveSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use interactive::pb::UserCollectsBiz;
use std::collections::HashMap;

pub struct NoteCollectedLoader {
    interactive_srv: InteractiveSrv,
}

impl NoteCollectedLoader {
    pub fn new(interactive_srv: InteractiveSrv) -> Self {
        Self { interactive_srv }
    }
}

type NoteCollectedKey = (i64, i64);

impl Loader<NoteCollectedKey> for NoteCollectedLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[NoteCollectedKey],
    ) -> async_graphql::Result<HashMap<NoteCollectedKey, Self::Value>, Self::Error> {
        Ok(self
            .interactive_srv
            .batch_get_collected(UserCollectsBiz::UserCollectsNote, keys.to_vec())
            .await?)
    }
}
