use crate::dto::comment::{Comment, Reply};
use crate::AppState;
use async_graphql::{Context, Object, Result};
use comment::pb::comment::CommentBiz;

#[derive(Default)]
pub(crate) struct CommentQuery;

#[Object]
impl CommentQuery {
    pub async fn replies(
        &self,
        ctx: &Context<'_>,
        note_id: i64,
        min_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<Reply>> {
        let min_id = min_id.unwrap_or(i64::MIN);
        let state = ctx.data::<AppState>()?;

        let ret = state
            .comment_srv
            .find_comments(CommentBiz::CommentComment, note_id, min_id, limit)
            .await?;

        Ok(ret.iter().map(|c| c.clone().into()).collect())
    }

    pub async fn comments(
        &self,
        ctx: &Context<'_>,
        note_id: i64,
        min_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<Comment>> {
        let min_id = min_id.unwrap_or(i64::MAX);
        let state = ctx.data::<AppState>()?;

        let ret = state
            .comment_srv
            .find_comments(CommentBiz::CommentNote, note_id, min_id, limit)
            .await?;

        Ok(ret)
    }

    pub async fn comment(&self, ctx: &Context<'_>, id: i64) -> Result<Comment> {
        let state = ctx.data::<AppState>()?;

        let ret = state.comment_srv.get_comment_by_id(id).await?;

        Ok(ret)
    }
}
