use crate::pb::user::user_service_server::UserService;
use crate::pb::user::{
    CreateUserRequest, CreateUserResponse, GetUserByIdRequest, GetUserByIdResponse,
    SendRegisterEmailCodeRequest, SendRegisterEmailCodeResponse, UpdateUserRequest,
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
}
