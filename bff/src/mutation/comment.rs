use crate::dto::comment::{Comment, Reply};
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, Object};
use comment::pb::comment::CommentBiz;
use interactive::model::{NoteCommentMessage, NoteLikeMessage, NoteReadMessage};
use interactive::pb::UserLikesBiz;
use tracing::error;

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

        tokio::spawn(async move {
            let data = NoteCommentMessage {
                biz_id: note_id,
                user_id,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note comment message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                note_id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "NoteComment",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
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

        tokio::spawn(async move {
            let data = NoteCommentMessage {
                biz_id: note_id,
                user_id,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note comment message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                note_id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "NoteComment",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok(ret.into())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn like_comment(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let ret = state
            .interactive_srv
            .like(user_id, UserLikesBiz::UserLikesComment, id)
            .await?;

        tokio::spawn(async move {
            let data = NoteLikeMessage {
                biz_id: id,
                user_id,
                like: true,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note like message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "CommentLike",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
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

        tokio::spawn(async move {
            let data = NoteLikeMessage {
                biz_id: id,
                user_id,
                like: false,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note like message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "CommentLike",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok("success".to_string())
    }
}
