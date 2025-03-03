use crate::model::User;

pub mod user;

impl From<User> for user::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            fullname: user.fullname,
            avatar: user.avatar,
            bio: user.bio.unwrap_or_default(),
            created_at: Some(prost_types::Timestamp {
                seconds: user.created_at.timestamp(),
                nanos: 0,
            }),
            serial_number: user.serial_number,
        }
    }
}
