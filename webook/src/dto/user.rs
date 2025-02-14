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
