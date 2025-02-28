use crate::dto::note::Note;
use crate::dto::user::User;
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub(crate) struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn profile(&self, ctx: &Context<'_>, uid: Option<i64>) -> Result<User> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>();

        if uid.is_none() && user_id.is_err() {
            return Err("user_id is required".into());
        }

        let user_id = uid.unwrap_or(*user_id.unwrap_or_else(|_| &-1));

        let user = state.user_srv.get_user_by_id(user_id).await?;
        Ok(user)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn user_notes(
        &self,
        ctx: &Context<'_>,
        uid: Option<i64>,
        page_size: i64,
        cursor_id: Option<i64>,
    ) -> Result<Vec<Note>> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>();

        if uid.is_none() && user_id.is_err() {
            return Err("user_id is required".into());
        }

        let user_id = uid.unwrap_or(*user_id.unwrap_or_else(|_| &-1));

        let notes = state
            .note_srv
            .get_user_published_notes(page_size, cursor_id, user_id)
            .await?;

        Ok(notes)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn liked_notes(
        &self,
        ctx: &Context<'_>,
        uid: Option<i64>,
        page_size: i64,
        cursor_id: Option<i64>,
    ) -> Result<Vec<Note>> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>();

        if uid.is_none() && user_id.is_err() {
            return Err("user_id is required".into());
        }

        let user_id = uid.unwrap_or(*user_id.unwrap_or_else(|_| &-1));

        let note_ids = state
            .interactive_srv
            .get_user_liked_note_ids(user_id, page_size, cursor_id)
            .await?;
        let notes = state.note_srv.batch_get_published_notes(note_ids).await?;
        Ok(notes)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn collected_notes(
        &self,
        ctx: &Context<'_>,
        uid: Option<i64>,
        page_size: i64,
        cursor_id: Option<i64>,
    ) -> Result<Vec<Note>> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>();

        if uid.is_none() && user_id.is_err() {
            return Err("user_id is required".into());
        }

        let user_id = uid.unwrap_or(*user_id.unwrap_or_else(|_| &-1));

        let note_ids = state
            .interactive_srv
            .get_user_collected_note_ids(user_id, page_size, cursor_id)
            .await?;
        let notes = state.note_srv.batch_get_published_notes(note_ids).await?;
        Ok(notes)
    }
}
