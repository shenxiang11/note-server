use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "count_biz", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum CountBiz {
    NoteRead,
    NoteLike,
    NoteCollect,
    NoteComment,
}

impl TryFrom<i32> for CountBiz {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CountBiz::NoteRead),
            2 => Ok(CountBiz::NoteLike),
            3 => Ok(CountBiz::NoteCollect),
            4 => Ok(CountBiz::NoteComment),
            _ => Err("invalid count biz"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    pub id: i64,
    pub biz: CountBiz,
    pub biz_id: i64,
    pub count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteReadMessage {
    pub biz_id: i64,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteCommentMessage {
    pub biz_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "history_biz", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum UserHistoryBiz {
    Note,
}

impl TryFrom<i32> for UserHistoryBiz {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserHistoryBiz::Note),
            _ => Err("invalid count biz"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Copy, Eq)]
#[sqlx(type_name = "user_likes_biz", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum UserLikesBiz {
    Note,
    Comment,
}

impl TryFrom<i32> for UserLikesBiz {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserLikesBiz::Note),
            2 => Ok(UserLikesBiz::Comment),
            _ => Err("invalid count biz"),
        }
    }
}
