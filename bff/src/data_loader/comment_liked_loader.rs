use crate::service::interactive::InteractiveSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use interactive::pb::UserLikesBiz;
use std::collections::HashMap;

pub struct CommentLikedLoader {
    interactive_srv: InteractiveSrv,
}

impl CommentLikedLoader {
    pub fn new(interactive_srv: InteractiveSrv) -> Self {
        Self { interactive_srv }
    }
}

type CommentLikedKey = (i64, i64);

impl Loader<CommentLikedKey> for CommentLikedLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[CommentLikedKey],
    ) -> async_graphql::Result<HashMap<CommentLikedKey, Self::Value>, Self::Error> {
        Ok(self
            .interactive_srv
            .batch_get_liked(UserLikesBiz::UserLikesComment, keys.to_vec())
            .await?)
    }
}
