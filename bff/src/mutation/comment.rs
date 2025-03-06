use crate::dto::comment::{Comment, Reply};
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, Object};
use comment::pb::comment::CommentBiz;
use interactive::pb::{CountBiz, UserLikesBiz};

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
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let ret = state
            .comment_srv
            .create(
                user_id,
                CommentBiz::CommentNote,
                note_id,
                None,
                None,
                content,
            )
            .await?;

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .increase_count(CountBiz::CountNoteComment, note_id)
                .await
        });

        Ok(ret)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn create_reply(
        &self,
        ctx: &Context<'_>,
        note_id: i64,
        root_id: i64,
        comment_id: i64,
        content: String,
    ) -> async_graphql::Result<Reply> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let ret = state
            .comment_srv
            .create(
                user_id,
                CommentBiz::CommentComment,
                note_id,
                Some(root_id),
                Some(comment_id),
                content,
            )
            .await?;

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .increase_count(CountBiz::CountNoteComment, note_id)
                .await
        });

        Ok(ret.into())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn like_comment(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let _ = state
            .interactive_srv
            .like(user_id, UserLikesBiz::UserLikesComment, id)
            .await?;

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .increase_count(CountBiz::CountCommentLike, id)
                .await
        });

        Ok("success".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn unlike_comment(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let _ = state
            .interactive_srv
            .unlike(user_id, UserLikesBiz::UserLikesComment, id)
            .await?;

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .decrease_count(CountBiz::CountCommentLike, id)
                .await
        });

        Ok("success".to_string())
    }
}
