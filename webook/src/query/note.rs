use crate::dto::note::Note;
use crate::AppState;
use async_graphql::{Context, Object, Result};
use interactive::model::NoteReadMessage;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

#[derive(Default)]
pub(crate) struct NoteQuery;

#[Object]
impl NoteQuery {
    pub async fn published_notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let state = ctx.data::<AppState>()?;
        let notes = state.note_srv.get_published_notes().await?;
        Ok(notes)
    }

    pub async fn published_note(&self, ctx: &Context<'_>, id: i64) -> Result<Note> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>();
        let note = state.note_srv.get_published_note_by_id(id).await?;

        let state = state.clone();
        let user_id = if user_id.is_ok() {
            Some(*user_id?)
        } else {
            None
        };

        tokio::spawn(async move {
            let data = NoteReadMessage {
                biz_id: note.id,
                user_id,
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize note read message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                note.id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "NoteRead",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok(note)
    }
}
