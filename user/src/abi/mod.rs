use crate::pb::user::{
    BatchGetUsersRequest, BatchGetUsersResponse, CreateUserRequest, CreateUserResponse,
    FollowUserRequest, FollowUserResponse, GetUserByIdRequest, GetUserByIdResponse,
    SendRegisterEmailCodeRequest, SendRegisterEmailCodeResponse, UnfollowUserRequest,
    UnfollowUserResponse, UpdateUserRequest, UpdateUserResponse, VerifyRequest, VerifyResponse,
};
use crate::{pb, UserSrv};
use tonic::{Response, Status};
use tracing::debug;

impl UserSrv {
    pub async fn send_register_email_code(
        &self,
        request: SendRegisterEmailCodeRequest,
    ) -> Result<Response<SendRegisterEmailCodeResponse>, Status> {
        let code = self.user_repo.send_email_code(&request.email).await?;
        debug!("email code sent: {}", code);
        Ok(Response::new(SendRegisterEmailCodeResponse {}))
    }

    pub async fn verify(&self, request: VerifyRequest) -> Result<Response<VerifyResponse>, Status> {
        let user = self
            .user_repo
            .verify(&request.email, &request.password)
            .await?;
        Ok(Response::new(VerifyResponse {
            user: Some(user.into()),
        }))
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let user = self
            .user_repo
            .create_user(&request.email, &request.password, &request.code)
            .await?;
        Ok(Response::new(CreateUserResponse {
            user: Some(user.into()),
        }))
    }

    pub async fn get_user_by_id(
        &self,
        request: GetUserByIdRequest,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
        let user = self.user_repo.get_user_by_id(request.id).await?;
        Ok(Response::new(GetUserByIdResponse {
            user: Some(user.into()),
        }))
    }

    pub async fn update_user(
        &self,
        request: UpdateUserRequest,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let _ = self
            .user_repo
            .update(request.id, request.fullname, request.avatar, request.bio)
            .await?;
        Ok(Response::new(UpdateUserResponse {}))
    }

    pub async fn batch_get_users(
        &self,
        request: BatchGetUsersRequest,
    ) -> Result<Response<BatchGetUsersResponse>, Status> {
        let users = self.user_repo.batch_get_users(request.ids).await?;
        Ok(Response::new(BatchGetUsersResponse {
            user: users.into_iter().map(|u| (u.0, u.1.into())).collect(),
        }))
    }

    pub async fn follow_user(
        &self,
        request: FollowUserRequest,
    ) -> Result<Response<FollowUserResponse>, Status> {
        let _ = self
            .user_repo
            .follow_user(request.follower, request.followee)
            .await?;
        Ok(Response::new(FollowUserResponse {}))
    }

    pub async fn unfollow_user(
        &self,
        request: UnfollowUserRequest,
    ) -> Result<Response<UnfollowUserResponse>, Status> {
        let _ = self
            .user_repo
            .unfollow_user(request.follower, request.followee)
            .await?;
        Ok(Response::new(UnfollowUserResponse {}))
    }
}
