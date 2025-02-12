use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "note_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum NoteType {
    Normal,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "note_status", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum NoteStatus {
    Draft,
    Published,
}

impl TryFrom<i32> for NoteStatus {
    type Error = &'static str;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(NoteStatus::Draft),
            2 => Ok(NoteStatus::Published),
            _ => Err("invalid note status"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "published_note_status", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum PublishedNoteStatus {
    Published,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
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
