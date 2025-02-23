use crate::dto::note::Note;
use crate::dto::user::User;
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub(crate) struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "AuthGuard")]
    pub async fn profile(&self, ctx: &Context<'_>) -> Result<User> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;

        let user = state.user_srv.get_user_by_id(*user_id).await?;
        Ok(user)
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn user_published_notes(
        &self,
        ctx: &Context<'_>,
        page_size: i64,
        cursor_id: Option<i64>,
    ) -> Result<Vec<Note>> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;

        let notes = state
            .note_srv
            .get_user_published_notes(page_size, cursor_id, *user_id)
            .await?;
        Ok(notes)
    }
}
