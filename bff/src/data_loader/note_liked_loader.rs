use crate::service::interactive::InteractiveSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use interactive::pb::UserLikesBiz;
use std::collections::HashMap;

pub struct NoteLikedLoader {
    interactive_srv: InteractiveSrv,
}

impl NoteLikedLoader {
    pub fn new(interactive_srv: InteractiveSrv) -> Self {
        Self { interactive_srv }
    }
}

type NoteLikedKey = (i64, i64);

impl Loader<NoteLikedKey> for NoteLikedLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[NoteLikedKey],
    ) -> async_graphql::Result<HashMap<NoteLikedKey, Self::Value>, Self::Error> {
        Ok(self
            .interactive_srv
            .batch_get_liked(UserLikesBiz::UserLikesNote, keys.to_vec())
            .await?)
    }
}
