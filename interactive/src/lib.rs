use crate::pb::interactive_service_server::InteractiveService;
use crate::pb::{
    BatchGetCountRequest, BatchGetCountResponse, BatchGetIsLikedRequest, BatchGetIsLikedResponse,
    GetCountRequest, GetCountResponse, LikeRequest, LikeResponse, SaveCountRequest,
    SaveCountResponse, UnlikeRequest, UnlikeResponse,
};
use crate::repository::InteractiveRepo;
use tonic::{async_trait, IntoRequest, Request, Response, Status};

mod abi;
pub mod config;
pub mod consumer;
pub mod model;
pub mod pb;
pub mod repository;

pub struct InteractiveSrv {
    interactive_repo: InteractiveRepo,
}

impl InteractiveSrv {
    pub fn new(interactive_repo: InteractiveRepo) -> Self {
        Self { interactive_repo }
    }
}

#[async_trait]
impl InteractiveService for InteractiveSrv {
    async fn save_count(
        &self,
        request: Request<SaveCountRequest>,
    ) -> Result<Response<SaveCountResponse>, Status> {
        self.save_count(request.into_inner()).await
    }

    async fn get_count(
        &self,
        request: Request<GetCountRequest>,
    ) -> Result<Response<GetCountResponse>, Status> {
        self.get_count(request.into_inner()).await
    }

    async fn batch_get_count(
        &self,
        request: Request<BatchGetCountRequest>,
    ) -> Result<Response<BatchGetCountResponse>, Status> {
        self.batch_get_count(request.into_inner()).await
    }

    async fn like(&self, request: Request<LikeRequest>) -> Result<Response<LikeResponse>, Status> {
        self.like(request.into_inner()).await
    }

    async fn unlike(
        &self,
        request: Request<UnlikeRequest>,
    ) -> Result<Response<UnlikeResponse>, Status> {
        self.unlike(request.into_inner()).await
    }

    async fn batch_get_is_liked(
        &self,
        request: Request<BatchGetIsLikedRequest>,
    ) -> Result<Response<BatchGetIsLikedResponse>, Status> {
        self.batch_get_is_liked(request.into_inner()).await
    }
}
