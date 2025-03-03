use crate::data_loader::comment_liked_loader::CommentLikedLoader;
use crate::data_loader::comment_likes_count_loader::CommentLikesCountLoader;
use crate::data_loader::comment_replies_loader::CommentRepliesLoader;
use crate::data_loader::replies_count_loader::RepliesCountLoader;
use crate::data_loader::reply_parent_loader::ReplyParentLoader;
use crate::data_loader::users_loader::UsersLoader;
use crate::dto::user::User;
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
    pub biz_id: i64,
    pub root_id: Option<i64>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub biz_id: i64,
    pub root_id: Option<i64>,
    pub parent_id: Option<i64>,
}

impl From<Comment> for Reply {
    fn from(value: Comment) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
            biz_id: value.biz_id,
            root_id: value.root_id,
            parent_id: value.parent_id,
        }
    }
}

#[ComplexObject]
impl Comment {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.data::<DataLoader<UsersLoader>>()?;
        let ret = loader.load_one(self.user_id).await?;

        Ok(ret)
    }

    pub async fn replies(&self, ctx: &Context<'_>) -> Result<Vec<Reply>> {
        let loader = ctx.data::<DataLoader<CommentRepliesLoader>>()?;
        let ret = loader.load_one(self.id).await?;

        Ok(ret
            .unwrap_or_default()
            .iter()
            .map(|c| c.clone().into())
            .collect())
    }

    pub async fn replies_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let loader = ctx.data::<DataLoader<RepliesCountLoader>>()?;
        let ret = loader.load_one(self.id).await?;

        Ok(ret.unwrap_or_default())
    }

    pub async fn liked_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let loader = ctx.data::<DataLoader<CommentLikesCountLoader>>()?;
        let ret = loader.load_one(self.id).await?;

        Ok(ret.unwrap_or_default())
    }

    pub async fn liked(&self, ctx: &Context<'_>) -> Result<bool> {
        let user_id = ctx.data::<i64>();
        match user_id {
            Ok(user_id) => {
                let loader = ctx.data::<DataLoader<CommentLikedLoader>>()?;
                let ret = loader.load_one((self.id, *user_id)).await?;
                Ok(ret.unwrap_or_default())
            }
            Err(_) => Ok(false),
        }
    }
}

#[ComplexObject]
impl Reply {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        let loader = ctx.data::<DataLoader<UsersLoader>>()?;
        let ret = loader.load_one(self.user_id).await?;

        Ok(ret)
    }

    pub async fn liked_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let loader = ctx.data::<DataLoader<CommentLikesCountLoader>>()?;
        let ret = loader.load_one(self.id).await?;

        Ok(ret.unwrap_or_default())
    }

    pub async fn liked(&self, ctx: &Context<'_>) -> Result<bool> {
        let user_id = ctx.data::<i64>();
        match user_id {
            Ok(user_id) => {
                let loader = ctx.data::<DataLoader<CommentLikedLoader>>()?;
                let ret = loader.load_one((self.id, *user_id)).await?;
                Ok(ret.unwrap_or_default())
            }
            Err(_) => Ok(false),
        }
    }

    pub async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Reply>> {
        let id = match self.parent_id {
            Some(id) => id,
            None => return Ok(None),
        };
        let loader = ctx.data::<DataLoader<ReplyParentLoader>>()?;
        let ret = loader.load_one(id).await?;
        Ok(ret)
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
            biz_id: value.biz_id,
            root_id: value.root_id,
            parent_id: value.parent_id,
        }
    }
}
