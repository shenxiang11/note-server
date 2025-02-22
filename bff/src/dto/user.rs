use crate::util::time::PbTimestamp;
use crate::AppState;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::{DateTime, Utc};
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

    pub async fn liked_count(&self) -> i64 {
        0
    }

    pub async fn collected_count(&self) -> i64 {
        0
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
