use crate::dto::comment::Reply;
use crate::service::comment::CommentSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use std::collections::HashMap;

pub struct ReplyParentLoader {
    comment_srv: CommentSrv,
}

impl ReplyParentLoader {
    pub fn new(comment_srv: CommentSrv) -> Self {
        Self { comment_srv }
    }
}

impl Loader<i64> for ReplyParentLoader {
    type Value = Reply;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[i64],
    ) -> async_graphql::Result<HashMap<i64, Self::Value>, Self::Error> {
        let ret = self
            .comment_srv
            .batch_get_comments_by_ids(keys.to_vec())
            .await?;

        Ok(ret)
    }
}
