use crate::data_loader::note_comments_count_loader::NoteCommentsCountLoader;
use crate::data_loader::note_views_loader::NoteViewsLoader;
// use crate::dto::user::User;
use crate::data_loader::users_loader::UsersLoader;
use crate::dto::user::User;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq, Enum,
)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum NoteStatus {
    Draft = 1,
    Published = 2,
    Hidden = 3,
}

impl From<i32> for NoteStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => NoteStatus::Draft,
            2 => NoteStatus::Published,
            3 => NoteStatus::Hidden,
            _ => panic!("invalid note status"),
        }
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq, Enum,
)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum NoteType {
    Normal = 1,
    Video = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub r#type: NoteType,
    pub status: NoteStatus,
    pub title: String,
    pub content: String,
    pub images: Vec<String>,
    pub video: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Note {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.data::<DataLoader<UsersLoader>>()?;
        let ret = loader.load_one(self.user_id).await?;

        Ok(ret)
    }

    pub async fn comments_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let loader = ctx.data::<DataLoader<NoteCommentsCountLoader>>()?;
        let ret = loader.load_one(self.id).await?;
        Ok(ret.unwrap_or_default())
    }

    pub async fn views(&self, ctx: &Context<'_>) -> Result<i64> {
        let loader = ctx.data::<DataLoader<NoteViewsLoader>>()?;
        let ret = loader.load_one(self.id).await?;
        Ok(ret.unwrap_or_default())
    }
}
