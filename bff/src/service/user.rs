use crate::dto::user::User;
use anyhow::anyhow;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tonic::transport::Channel;
use user::pb::user::user_service_client::UserServiceClient;
use user::pb::user::{BatchGetIsFollowedRequest, SendRegisterEmailCodeRequest};

#[derive(Clone)]
pub struct UserSrv {
    inner: Arc<UserSrvInner>,
}

impl Deref for UserSrv {
    type Target = UserSrvInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct UserSrvInner {
    client: UserServiceClient<Channel>,
}

impl UserSrv {
    pub fn new(client: UserServiceClient<Channel>) -> Self {
        Self {
            inner: Arc::new(UserSrvInner { client }),
        }
    }

    pub async fn batch_get_users(&self, ids: Vec<i64>) -> anyhow::Result<HashMap<i64, User>> {
        let mut client = self.client.clone();
        let resp = client
            .batch_get_users(user::pb::user::BatchGetUsersRequest { ids })
            .await?
            .into_inner();

        Ok(resp
            .user
            .into_iter()
            .map(|user| (user.0, user.1.into()))
            .collect())
    }

    // 发送邮箱验证码
    // 同一个邮箱，一分钟以内只能发送一次
    // 验证码有效期 10 分钟
    // 验证通过，或者验证三次失败，验证码失效
    pub async fn send_register_email_code(&self, email: String) -> anyhow::Result<()> {
        let mut client = self.client.clone();
        let _ = client
            .send_register_email_code(SendRegisterEmailCodeRequest { email })
            .await?
            .into_inner();

        Ok(())
    }

    pub async fn create_user(
        &self,
        email: String,
        password: String,
        code: String,
    ) -> anyhow::Result<User> {
        let mut client = self.client.clone();
        let resp = client
            .create_user(user::pb::user::CreateUserRequest {
                email,
                password,
                code,
            })
            .await?
            .into_inner();

        match resp.user {
            Some(user) => Ok(user.into()),
            None => Err(anyhow!("user not found")),
        }
    }

    pub async fn verify(&self, email: String, password: String) -> anyhow::Result<User> {
        let mut client = self.client.clone();
        let resp = client
            .verify(user::pb::user::VerifyRequest { email, password })
            .await?
            .into_inner();

        match resp.user {
            Some(user) => Ok(user.into()),
            None => Err(anyhow!("user not found")),
        }
    }

    pub async fn update_user(
        &self,
        id: i64,
        fullname: Option<String>,
        avatar: Option<String>,
        bio: Option<String>,
    ) -> anyhow::Result<()> {
        let mut client = self.client.clone();
        let _ = client
            .update_user(user::pb::user::UpdateUserRequest {
                id,
                fullname,
                avatar,
                bio,
            })
            .await?
            .into_inner();

        Ok(())
    }

    pub async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let mut client = self.client.clone();
        let resp = client
            .get_user_by_id(user::pb::user::GetUserByIdRequest { id })
            .await?
            .into_inner();

        match resp.user {
            Some(user) => Ok(user.into()),
            None => Err(anyhow!("user not found")),
        }
    }

    pub async fn follow_user(&self, follower: i64, followee: i64) -> anyhow::Result<()> {
        let mut client = self.client.clone();
        let _ = client
            .follow_user(user::pb::user::FollowUserRequest { follower, followee })
            .await?
            .into_inner();

        Ok(())
    }

    pub async fn unfollow_user(&self, follower: i64, followee: i64) -> anyhow::Result<()> {
        let mut client = self.client.clone();
        let _ = client
            .unfollow_user(user::pb::user::UnfollowUserRequest { follower, followee })
            .await?
            .into_inner();

        Ok(())
    }

    pub async fn get_followers_count(&self, user_id: i64) -> anyhow::Result<i64> {
        let mut client = self.client.clone();
        let resp = client
            .get_follows_count(user::pb::user::GetFollowsCountRequest { user_id })
            .await?
            .into_inner();

        Ok(resp.count)
    }

    pub async fn get_fans_count(&self, user_id: i64) -> anyhow::Result<i64> {
        let mut client = self.client.clone();
        let resp = client
            .get_fans_count(user::pb::user::GetFansCountRequest { user_id })
            .await?
            .into_inner();

        Ok(resp.count)
    }

    pub async fn batch_get_is_followed(
        &self,
        is_followed_queries: Vec<(i64, i64)>,
    ) -> anyhow::Result<HashMap<(i64, i64), bool>> {
        let mut client = self.client.clone();
        let resp = client
            .batch_get_is_followed(BatchGetIsFollowedRequest {
                query: is_followed_queries
                    .into_iter()
                    .map(|(follower, followee)| user::pb::user::IsFollowedQuery {
                        follower,
                        followee,
                    })
                    .collect(),
            })
            .await?
            .into_inner();

        Ok(resp
            .result
            .iter()
            .map(|r| ((r.follower, r.followee), r.is_followed))
            .collect())
    }
}
