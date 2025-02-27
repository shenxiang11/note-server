use crate::pb::user::user_service_server::UserService;
use crate::pb::user::{
    BatchGetIsFollowedRequest, BatchGetIsFollowedResponse, BatchGetUsersRequest,
    BatchGetUsersResponse, CreateUserRequest, CreateUserResponse, FollowUserRequest,
    FollowUserResponse, GetFansCountRequest, GetFansCountResponse, GetFollowsCountRequest,
    GetFollowsCountResponse, GetUserByIdRequest, GetUserByIdResponse, SendRegisterEmailCodeRequest,
    SendRegisterEmailCodeResponse, UnfollowUserRequest, UnfollowUserResponse, UpdateUserRequest,
    UpdateUserResponse, VerifyRequest, VerifyResponse,
};
use crate::repository::UserRepo;
use tonic::{async_trait, Request, Response, Status};

mod abi;
pub mod config;
pub mod error;
mod model;
pub mod pb;
pub mod repository;

pub struct UserSrv {
    user_repo: UserRepo,
}

impl UserSrv {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserService for UserSrv {
    async fn send_register_email_code(
        &self,
        request: Request<SendRegisterEmailCodeRequest>,
    ) -> Result<Response<SendRegisterEmailCodeResponse>, Status> {
        self.send_register_email_code(request.into_inner()).await
    }

    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        self.verify(request.into_inner()).await
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        self.create_user(request.into_inner()).await
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
        self.get_user_by_id(request.into_inner()).await
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        self.update_user(request.into_inner()).await
    }

    async fn batch_get_users(
        &self,
        request: Request<BatchGetUsersRequest>,
    ) -> Result<Response<BatchGetUsersResponse>, Status> {
        self.batch_get_users(request.into_inner()).await
    }

    async fn follow_user(
        &self,
        request: Request<FollowUserRequest>,
    ) -> Result<Response<FollowUserResponse>, Status> {
        self.follow_user(request.into_inner()).await
    }

    async fn unfollow_user(
        &self,
        request: Request<UnfollowUserRequest>,
    ) -> Result<Response<UnfollowUserResponse>, Status> {
        self.unfollow_user(request.into_inner()).await
    }

    async fn get_follows_count(
        &self,
        request: Request<GetFollowsCountRequest>,
    ) -> Result<Response<GetFollowsCountResponse>, Status> {
        self.get_follows_count(request.into_inner()).await
    }

    async fn get_fans_count(
        &self,
        request: Request<GetFansCountRequest>,
    ) -> Result<Response<GetFansCountResponse>, Status> {
        self.get_fans_count(request.into_inner()).await
    }

    async fn batch_get_is_followed(
        &self,
        request: Request<BatchGetIsFollowedRequest>,
    ) -> Result<Response<BatchGetIsFollowedResponse>, Status> {
        self.batch_get_is_followed(request.into_inner()).await
    }
}
