use crate::pb::interactive_service_server::InteractiveService;
use crate::pb::{
    BatchGetCountRequest, BatchGetCountResponse, BatchGetIsCollectedRequest,
    BatchGetIsCollectedResponse, BatchGetIsLikedRequest, BatchGetIsLikedResponse, CollectRequest,
    CollectResponse, DecreaseCountRequest, DecreaseCountResponse, GetCountRequest,
    GetCountResponse, GetUserCollectedNoteIdsRequest, GetUserCollectedNoteIdsResponse,
    GetUserLikedNoteIdsRequest, GetUserLikedNoteIdsResponse, IncreaseCountRequest,
    IncreaseCountResponse, LikeRequest, LikeResponse, SaveCountRequest, SaveCountResponse,
    UncollectRequest, UncollectResponse, UnlikeRequest, UnlikeResponse,
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
    async fn increase_count(
        &self,
        request: Request<IncreaseCountRequest>,
    ) -> Result<Response<IncreaseCountResponse>, Status> {
        self.increase_count(request.into_inner()).await
    }

    async fn decrease_count(
        &self,
        request: Request<DecreaseCountRequest>,
    ) -> Result<Response<DecreaseCountResponse>, Status> {
        self.decrease_count(request.into_inner()).await
    }

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

    async fn batch_get_is_collected(
        &self,
        request: Request<BatchGetIsCollectedRequest>,
    ) -> Result<Response<BatchGetIsCollectedResponse>, Status> {
        self.batch_get_is_collected(request.into_inner()).await
    }

    async fn collect(
        &self,
        request: Request<CollectRequest>,
    ) -> Result<Response<CollectResponse>, Status> {
        self.collect(request.into_inner()).await
    }

    async fn uncollect(
        &self,
        request: Request<UncollectRequest>,
    ) -> Result<Response<UncollectResponse>, Status> {
        self.uncollect(request.into_inner()).await
    }

    async fn get_user_liked_note_ids(
        &self,
        request: Request<GetUserLikedNoteIdsRequest>,
    ) -> Result<Response<GetUserLikedNoteIdsResponse>, Status> {
        self.get_user_liked_note_ids(request.into_inner()).await
    }

    async fn get_user_collected_note_ids(
        &self,
        request: Request<GetUserCollectedNoteIdsRequest>,
    ) -> Result<Response<GetUserCollectedNoteIdsResponse>, Status> {
        self.get_user_collected_note_ids(request.into_inner()).await
    }
}
