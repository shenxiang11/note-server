use crate::data_loader::user_followed_loader::UserFollowedLoader;
use crate::util::time::PbTimestamp;
use crate::AppState;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::{DateTime, Utc};
use interactive::pb::CountBiz;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub serial_number: String,
    pub fullname: String,
    pub email: String,
    pub avatar: String,
    pub bio: String,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl User {
    pub async fn is_self(&self, ctx: &Context<'_>) -> Result<bool> {
        let user_id = ctx.data::<i64>()?;

        Ok(*user_id == self.id)
    }

    pub async fn is_followed(&self, ctx: &Context<'_>) -> Result<bool> {
        let user_id = ctx.data::<i64>();

        match user_id {
            Ok(user_id) => {
                let loader = ctx.data::<DataLoader<UserFollowedLoader>>()?;
                let ret = loader.load_one((*user_id, self.id)).await?;
                Ok(ret.unwrap_or_default())
            }
            Err(_) => Ok(false),
        }
    }

    pub async fn follows_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<AppState>()?;
        let n = state
            .user_srv
            .get_followers_count(self.id)
            .await
            .unwrap_or_default();

        Ok(n)
    }

    pub async fn fans_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<AppState>()?;
        let n = state
            .user_srv
            .get_fans_count(self.id)
            .await
            .unwrap_or_default();
        Ok(n)
    }

    pub async fn liked_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<AppState>()?;

        let note_ids = state.note_srv.get_published_note_ids_by_user(self.id).await;
        if note_ids.is_err() {
            return Ok(0);
        }
        let note_ids = note_ids.unwrap_or_default();
        let ret = state
            .interactive_srv
            .batch_get_count(CountBiz::CountNoteLike, note_ids)
            .await;
        if ret.is_err() {
            return Ok(0);
        }
        let ret = ret.unwrap_or_default();
        Ok(ret.values().fold(0, |acc, x| acc + x))
    }

    pub async fn collected_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let state = ctx.data::<AppState>()?;

        let note_ids = state.note_srv.get_published_note_ids_by_user(self.id).await;
        if note_ids.is_err() {
            return Ok(0);
        }
        let note_ids = note_ids.unwrap_or_default();
        let ret = state
            .interactive_srv
            .batch_get_count(CountBiz::CountNoteCollect, note_ids)
            .await;
        if ret.is_err() {
            return Ok(0);
        }
        let ret = ret.unwrap_or_default();
        Ok(ret.values().fold(0, |acc, x| acc + x))
    }
}

impl From<user::pb::user::User> for User {
    fn from(value: user::pb::user::User) -> Self {
        Self {
            id: value.id,
            serial_number: "".to_string(),
            fullname: value.fullname,
            email: value.email,
            avatar: value.avatar,
            bio: value.bio,
            created_at: PbTimestamp(value.created_at.unwrap_or_default()).into(),
        }
    }
}
