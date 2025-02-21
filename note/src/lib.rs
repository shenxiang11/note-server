use crate::pb::note::note_service_server::NoteService;
use crate::pb::note::{
    CreateOrUpdateRequest, CreateOrUpdateResponse, GetPublishedNoteRequest,
    GetPublishedNoteResponse, GetPublishedNotesRequest, GetPublishedNotesResponse,
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
    async fn get_published_notes(
        &self,
        request: Request<GetPublishedNotesRequest>,
    ) -> Result<Response<GetPublishedNotesResponse>, Status> {
        self.get_published_notes(request.into_inner()).await
    }

    async fn create_or_update(
        &self,
        request: Request<CreateOrUpdateRequest>,
    ) -> Result<Response<CreateOrUpdateResponse>, Status> {
        self.create_or_update(request.into_inner()).await
    }

    async fn get_published_note(
        &self,
        request: Request<GetPublishedNoteRequest>,
    ) -> Result<Response<GetPublishedNoteResponse>, Status> {
        self.get_published_note(request.into_inner()).await
    }
}
