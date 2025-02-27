use crate::service::user::UserSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use std::collections::HashMap;

pub struct UserFollowedLoader {
    user_srv: UserSrv,
}

impl UserFollowedLoader {
    pub fn new(user_srv: UserSrv) -> Self {
        Self { user_srv }
    }
}

type UserFollowedKey = (i64, i64);

impl Loader<UserFollowedKey> for UserFollowedLoader {
    type Value = bool;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[UserFollowedKey],
    ) -> async_graphql::Result<HashMap<UserFollowedKey, Self::Value>, Self::Error> {
        Ok(self.user_srv.batch_get_is_followed(keys.to_vec()).await?)
    }
}
