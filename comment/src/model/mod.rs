use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq, Enum,
)]
#[sqlx(type_name = "comment_biz", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum CommentBiz {
    Note,
    Comment,
}

impl TryFrom<i32> for CommentBiz {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CommentBiz::Note),
            2 => Ok(CommentBiz::Comment),
            _ => Err("invalid comment biz"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i64,
    pub user_id: i64,
    pub biz: CommentBiz,
    pub biz_id: i64,
    pub root_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// impl Comment {
//     pub fn to_pb(&self) {
//         let _ = CommentBiz::Comment;
//         // CommentPb {
//         //     id: self.id,
//         //     user_id: self.user_id,
//         //     biz: self.biz as i32,
//         //     biz_id: self.biz_id,
//         //     root_id: self.root_id,
//         //     parent_id: self.parent_id,
//         //     content: self.content.clone(),
//         //     created_at: Some(prost_types::Timestamp {
//         //         seconds: self.created_at.timestamp(),
//         //         nanos: 0,
//         //     }),
//         //     updated_at: Some(prost_types::Timestamp {
//         //         seconds: self.updated_at.timestamp(),
//         //         nanos: 0,
//         //     }),
//         // }
//     }
// }
