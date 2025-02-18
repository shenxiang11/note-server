use crate::util::time::PbTimestamp;
use async_graphql::{ComplexObject, SimpleObject};
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
impl User {}

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
