use crate::dto::note::NoteStatus;
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, InputObject, Object};
use interactive::model::{
    NoteCollectMessage, NoteCommentMessage, NoteLikeMessage, UserCollectsBiz,
};
use interactive::pb::UserCollectsBiz::UserCollectsNote;
use interactive::pb::UserLikesBiz;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Default)]
pub(crate) struct NoteMutation;

#[Object]
impl NoteMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn create_note(
        &self,
        ctx: &Context<'_>,
        input: EditNoteInput,
    ) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;

        let note_id = state
            .note_srv
            .create_or_update_draft(*user_id, None, input.clone())
            .await?;

        if input.direct_publish {
            state.note_srv.publish_note(*user_id, note_id).await?;
        }

        Ok("".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn edit_note(
        &self,
        ctx: &Context<'_>,
        note_id: i64,
        input: EditNoteInput,
    ) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;

        let note_id = state
            .note_srv
            .create_or_update_draft(*user_id, Some(note_id), input.clone())
            .await?;

        if input.direct_publish {
            state.note_srv.publish_note(*user_id, note_id).await?;
        }

        Ok("".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn collect_note(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let _ = state
            .interactive_srv
            .collect(user_id, UserCollectsNote, id)
            .await?;

        tokio::spawn(async move {
            let data = NoteCollectMessage {
                biz_id: id,
                user_id,
                collect: true,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note like message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "NoteCollect",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok("success".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn cancel_collect_note(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let _ = state
            .interactive_srv
            .uncollect(user_id, UserCollectsNote, id)
            .await?;

        tokio::spawn(async move {
            let data = NoteCollectMessage {
                biz_id: id,
                user_id,
                collect: false,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note like message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "NoteCollect",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok("success".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn like_note(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let ret = state
            .interactive_srv
            .like(user_id, UserLikesBiz::UserLikesNote, id)
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
                "NoteLike",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok("success".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn unlike_note(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let _ = state
            .interactive_srv
            .unlike(user_id, UserLikesBiz::UserLikesNote, id)
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
                "NoteLike",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok("success".to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct EditNoteInput {
    pub title: Option<String>,
    pub content: Option<String>,
    pub images: Option<Vec<String>>,
    pub video: Option<String>,
    pub direct_publish: bool,
}
