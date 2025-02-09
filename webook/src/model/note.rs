use crate::service::interactive::InteractiveSrv;
use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::{ComplexObject, Enum, FieldError, Result, SimpleObject};
use chrono::{DateTime, Utc};
use interactive::pb::CountBiz;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

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

pub struct PublishedNoteViewsLoader {
    interactive_srv: InteractiveSrv,
}

impl PublishedNoteViewsLoader {
    pub fn new(interactive_srv: InteractiveSrv) -> Self {
        Self { interactive_srv }
    }
}

impl Loader<i64> for PublishedNoteViewsLoader {
    type Value = i64;
    type Error = FieldError;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let ret = self
            .interactive_srv
            .batch_get_count(CountBiz::CountNoteRead, keys.to_vec())
            .await;
        match ret {
            Ok(hm) => Ok(hm),
            Err(e) => Err(FieldError::from(e)),
        }
    }
}

#[ComplexObject]
impl PublishedNote {
    pub async fn views(&self, ctx: &async_graphql::Context<'_>) -> Result<i64> {
        let loader = ctx.data::<DataLoader<PublishedNoteViewsLoader>>()?;
        let ret = loader.load_one(self.id).await?;

        Ok(ret.unwrap_or_default())
    }
}
