use async_graphql::{ComplexObject, Enum, Result, SimpleObject};
use chrono::{DateTime, Utc};
use interactive::pb::CountBiz;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::error;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Enum, Copy, Eq,
)]
#[sqlx(type_name = "note_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum NoteType {
    Normal,
    Video,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Enum, Copy, Eq,
)]
#[sqlx(type_name = "note_status", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum NoteStatus {
    Draft,
    Published,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Enum, Copy, Eq,
)]
#[sqlx(type_name = "published_note_status", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum PublishedNoteStatus {
    Published,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub r#type: NoteType,
    pub status: NoteStatus,
    pub title: String,
    pub content: Option<String>,
    pub images: Vec<String>,
    #[sqlx(default)]
    pub video: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Note {}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct PublishedNote {
    pub id: i64,
    pub user_id: i64,
    pub r#type: NoteType,
    pub status: PublishedNoteStatus,
    pub title: String,
    pub content: Option<String>,
    pub images: Vec<String>,
    #[sqlx(default)]
    pub video: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl PublishedNote {
    pub async fn views(&self, ctx: &async_graphql::Context<'_>) -> Result<i64> {
        let state = ctx.data::<crate::AppState>()?;
        let ret = state
            .interactive_srv
            .get_count(CountBiz::CountNoteRead, self.id)
            .await;

        match ret {
            Ok(count) => Ok(count),
            Err(e) => {
                error!("failed to get views: {}", e);
                Ok(0)
            }
        }
    }
}
