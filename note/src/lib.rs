use crate::pb::note::note_service_server::NoteService;
use crate::pb::note::{
    BatchGetPublishedNotesRequest, BatchGetPublishedNotesResponse, CreateOrUpdateDraftRequest,
    CreateOrUpdateDraftResponse, GetPublishedNoteRequest, GetPublishedNoteResponse,
    GetPublishedNotesRequest, GetPublishedNotesResponse, GetUserPublishedNoteIdsRequest,
    GetUserPublishedNoteIdsResponse, GetUserPublishedNotesRequest, GetUserPublishedNotesResponse,
    PublishDraftNoteRequest, PublishDraftNoteResponse,
};
use crate::repository::NoteRepo;
use tonic::{async_trait, Request, Response, Status};

mod abi;
pub mod config;
pub mod model;
pub mod pb;
pub mod repository;

pub struct NoteSrv {
    note_repo: NoteRepo,
}

impl NoteSrv {
    pub fn new(note_repo: NoteRepo) -> Self {
        Self { note_repo }
    }
}

#[async_trait]
impl NoteService for NoteSrv {
    async fn create_or_update_draft_note(
        &self,
        request: Request<CreateOrUpdateDraftRequest>,
    ) -> Result<Response<CreateOrUpdateDraftResponse>, Status> {
        self.create_or_update_draft_note(request.into_inner()).await
    }

    async fn publish_draft_note(
        &self,
        request: Request<PublishDraftNoteRequest>,
    ) -> Result<Response<PublishDraftNoteResponse>, Status> {
        self.publish_draft_note(request.into_inner()).await
    }

    async fn get_published_note(
        &self,
        request: Request<GetPublishedNoteRequest>,
    ) -> Result<Response<GetPublishedNoteResponse>, Status> {
        self.get_published_note(request.into_inner()).await
    }

    async fn get_published_notes(
        &self,
        request: Request<GetPublishedNotesRequest>,
    ) -> Result<Response<GetPublishedNotesResponse>, Status> {
        self.get_published_notes(request.into_inner()).await
    }

    async fn get_user_published_note_ids(
        &self,
        request: Request<GetUserPublishedNoteIdsRequest>,
    ) -> Result<Response<GetUserPublishedNoteIdsResponse>, Status> {
        self.get_user_published_note_ids(request.into_inner()).await
    }

    async fn get_user_published_notes(
        &self,
        request: Request<GetUserPublishedNotesRequest>,
    ) -> Result<Response<GetUserPublishedNotesResponse>, Status> {
        self.get_user_published_notes(request.into_inner()).await
    }

    async fn batch_get_published_notes(
        &self,
        request: Request<BatchGetPublishedNotesRequest>,
    ) -> Result<Response<BatchGetPublishedNotesResponse>, Status> {
        self.batch_get_published_notes(request.into_inner()).await
    }
}
