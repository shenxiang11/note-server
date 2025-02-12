use crate::data_loader::comment_replies_loader::CommentRepliesLoader;
use crate::util::time::PbTimestamp;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use chrono::{DateTime, Utc};
use comment::pb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Comment {
    pub async fn top_two_replies(&self, ctx: &Context<'_>) -> Result<Vec<Comment>> {
        let loader = ctx.data::<DataLoader<CommentRepliesLoader>>()?;
        let ret = loader.load_one(self.id).await?;

        Ok(ret.unwrap_or_default())
    }
}

impl From<pb::comment::Comment> for Comment {
    fn from(value: pb::comment::Comment) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            content: value.content,
            created_at: PbTimestamp(value.created_at.unwrap_or_default()).into(),
            updated_at: PbTimestamp(value.updated_at.unwrap_or_default()).into(),
        }
    }
}
