use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, InputObject, Object};
use interactive::pb::UserCollectsBiz::UserCollectsNote;
use interactive::pb::{CountBiz, UserLikesBiz};
use serde::{Deserialize, Serialize};

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

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .increase_count(CountBiz::CountNoteCollect, id)
                .await
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

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .decrease_count(CountBiz::CountNoteCollect, id)
                .await
        });

        Ok("success".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn like_note(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();
        let _ = state
            .interactive_srv
            .like(user_id, UserLikesBiz::UserLikesNote, id)
            .await?;

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .increase_count(CountBiz::CountNoteLike, id)
                .await
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

        let state = state.clone();
        tokio::spawn(async move {
            state
                .interactive_srv
                .decrease_count(CountBiz::CountNoteLike, id)
                .await
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
