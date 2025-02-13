use crate::service::comment::CommentSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use std::collections::HashMap;

pub struct NoteCommentsCountLoader {
    comment_srv: CommentSrv,
}

impl NoteCommentsCountLoader {
    pub fn new(comment_srv: CommentSrv) -> Self {
        Self { comment_srv }
    }
}

impl Loader<i64> for NoteCommentsCountLoader {
    type Value = i64;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[i64],
    ) -> async_graphql::Result<HashMap<i64, Self::Value>, Self::Error> {
        Ok(self
            .comment_srv
            .batch_get_note_comments_count(keys.to_vec())
            .await?)
    }
}
