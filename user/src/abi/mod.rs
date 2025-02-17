use crate::pb::user::{
    CreateUserRequest, CreateUserResponse, GetUserByIdRequest, GetUserByIdResponse,
    SendRegisterEmailCodeRequest, SendRegisterEmailCodeResponse, UpdateUserRequest,
    UpdateUserResponse, VerifyRequest, VerifyResponse,
};
use crate::UserSrv;
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
        let _ = self
            .user_repo
            .verify(&request.email, &request.password)
            .await?;
        Ok(Response::new(VerifyResponse {}))
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let user = self
            .user_repo
            .create_user(&request.email, &request.password, &request.code)
            .await?;
        Ok(Response::new(CreateUserResponse {}))
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
}
