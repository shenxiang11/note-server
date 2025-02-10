use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, Object};
use comment::model::Comment;
use comment::pb::comment::CommentBiz;

#[derive(Default)]
pub(crate) struct CommentMutation;

#[Object]
impl CommentMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn create_note_comment(
        &self,
        ctx: &Context<'_>,
        note_id: i64,
        content: String,
    ) -> async_graphql::Result<Comment> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;
        let ret = state
            .comment_srv
            .create(
                *user_id,
                CommentBiz::CommentNote,
                note_id,
                None,
                None,
                content,
            )
            .await?;

        Ok(ret)
    }
}
