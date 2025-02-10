use async_graphql::{Context, Object, Result};
use comment::model::Comment;

#[derive(Default)]
pub(crate) struct CommentQuery;

#[Object]
impl CommentQuery {
    pub async fn comments(&self, ctx: &Context<'_>) -> Result<Comment> {
        unimplemented!()
    }
}
