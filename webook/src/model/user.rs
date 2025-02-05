use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, SimpleObject)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub serial_number: String,
    pub fullname: String,
    pub email: String,
    #[serde(skip)]
    #[graphql(skip)]
    pub password_hash: String,
    pub avatar: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl User {}
