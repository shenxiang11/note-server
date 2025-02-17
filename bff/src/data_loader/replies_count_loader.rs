use crate::service::comment::CommentSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use std::collections::HashMap;

pub struct RepliesCountLoader {
    comment_srv: CommentSrv,
}

impl RepliesCountLoader {
    pub fn new(comment_srv: CommentSrv) -> Self {
        Self { comment_srv }
    }
}

impl Loader<i64> for RepliesCountLoader {
    type Value = i64;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[i64],
    ) -> async_graphql::Result<HashMap<i64, Self::Value>, Self::Error> {
        Ok(self
            .comment_srv
            .batch_get_replies_count(keys.to_vec())
            .await?)
    }
}
