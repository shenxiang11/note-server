use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "comment_biz", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum CommentBiz {
    Note,
    Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
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
