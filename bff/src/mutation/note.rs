use crate::dto::note::NoteStatus;
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, InputObject, Object};
use interactive::pb::UserLikesBiz;
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

        let _ = state
            .note_srv
            .create_or_update(*user_id, None, input)
            .await?;

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

        let _ = state
            .note_srv
            .create_or_update(*user_id, Some(note_id), input)
            .await?;

        Ok("".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn like_note(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;
        state
            .interactive_srv
            .like(*user_id, UserLikesBiz::UserLikesNote, id)
            .await?;

        Ok("success".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn unlike_note(&self, ctx: &Context<'_>, id: i64) -> async_graphql::Result<String> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;
        state
            .interactive_srv
            .unlike(*user_id, UserLikesBiz::UserLikesNote, id)
            .await?;

        Ok("success".to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
pub struct EditNoteInput {
    pub title: Option<String>,
    pub content: Option<String>,
    pub images: Option<Vec<String>>,
    pub video: Option<String>,
    pub status: Option<NoteStatus>,
}
