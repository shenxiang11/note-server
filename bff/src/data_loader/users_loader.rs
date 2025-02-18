use crate::dto::user::User;
use crate::service::user::UserSrv;
use async_graphql::dataloader::Loader;
use async_graphql::FieldError;
use std::collections::HashMap;

pub struct UsersLoader {
    user_srv: UserSrv,
}

impl UsersLoader {
    pub fn new(user_srv: UserSrv) -> Self {
        Self { user_srv }
    }
}

impl Loader<i64> for UsersLoader {
    type Value = User;
    type Error = FieldError;

    async fn load(
        &self,
        keys: &[i64],
    ) -> async_graphql::Result<HashMap<i64, Self::Value>, Self::Error> {
        Ok(self.user_srv.batch_get_users(keys.to_vec()).await?)
    }
}
