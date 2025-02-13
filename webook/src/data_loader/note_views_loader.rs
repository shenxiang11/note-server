use crate::service::interactive::InteractiveSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use interactive::pb::CountBiz;
use std::collections::HashMap;

pub struct NoteViewsLoader {
    interactive_srv: InteractiveSrv,
}

impl NoteViewsLoader {
    pub fn new(interactive_srv: InteractiveSrv) -> Self {
        Self { interactive_srv }
    }
}

impl Loader<i64> for NoteViewsLoader {
    type Value = i64;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[i64],
    ) -> async_graphql::Result<HashMap<i64, Self::Value>, Self::Error> {
        let ret = self
            .interactive_srv
            .batch_get_count(CountBiz::CountNoteRead, keys.to_vec())
            .await;
        match ret {
            Ok(hm) => Ok(hm),
            Err(e) => Err(FieldError::from(e)),
        }
    }
}
