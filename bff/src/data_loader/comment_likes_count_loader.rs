use crate::service::interactive::InteractiveSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use interactive::pb::CountBiz;
use std::collections::HashMap;

pub struct CommentLikesCountLoader {
    interactive_srv: InteractiveSrv,
}

impl CommentLikesCountLoader {
    pub fn new(interactive_srv: InteractiveSrv) -> Self {
        Self { interactive_srv }
    }
}

impl Loader<i64> for CommentLikesCountLoader {
    type Value = i64;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[i64],
    ) -> async_graphql::Result<HashMap<i64, Self::Value>, Self::Error> {
        Ok(self
            .interactive_srv
            .batch_get_count(CountBiz::CountCommentLike, keys.to_vec())
            .await?)
    }
}
