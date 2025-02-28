use crate::model::NoteType;
use crate::pb::note::{
    get_published_note_response, BatchGetPublishedNotesRequest, BatchGetPublishedNotesResponse,
    CreateOrUpdateDraftRequest, CreateOrUpdateDraftResponse, GetPublishedNoteRequest,
    GetPublishedNoteResponse, GetPublishedNotesRequest, GetPublishedNotesResponse,
    GetUserPublishedNoteIdsRequest, GetUserPublishedNoteIdsResponse, GetUserPublishedNotesRequest,
    GetUserPublishedNotesResponse, ImageList, NormalNote, PublishDraftNoteRequest,
    PublishDraftNoteResponse, VideoNote,
};
use crate::NoteSrv;
use get_published_note_response::Note;
use tonic::{Response, Status};

impl NoteSrv {
    pub async fn batch_get_published_notes(
        &self,
        req: BatchGetPublishedNotesRequest,
    ) -> Result<Response<BatchGetPublishedNotesResponse>, Status> {
        let notes = self.note_repo.batch_get_published_notes(req.ids).await;

        match notes {
            Ok(notes) => {
                let notes = notes
                    .into_iter()
                    .map(|note| {
                        let note = if note.r#type == NoteType::Normal {
                            Note::NormalNote(NormalNote {
                                id: note.id,
                                title: note.title,
                                content: note.content.unwrap_or_default(),
                                images: Some(ImageList {
                                    images: note.images,
                                }),
                                status: note.status as i32,
                                user_id: note.user_id,
                                created_at: Some(prost_types::Timestamp {
                                    seconds: note.created_at.timestamp(),
                                    nanos: 0,
                                }),
                                updated_at: Some(prost_types::Timestamp {
                                    seconds: note.updated_at.timestamp(),
                                    nanos: 0,
                                }),
                            })
                        } else {
                            Note::VideoNote(VideoNote {
                                id: note.id,
                                title: note.title,
                                content: note.content.unwrap_or_default(),
                                images: Some(ImageList {
                                    images: note.images,
                                }),
                                video: note.video.unwrap_or_default(),
                                status: note.status as i32,
                                user_id: note.user_id,
                                created_at: Some(prost_types::Timestamp {
                                    seconds: note.created_at.timestamp(),
                                    nanos: 0,
                                }),
                                updated_at: Some(prost_types::Timestamp {
                                    seconds: note.updated_at.timestamp(),
                                    nanos: 0,
                                }),
                            })
                        };
                        GetPublishedNoteResponse { note: Some(note) }
                    })
                    .collect();

                let resp = BatchGetPublishedNotesResponse { notes };

                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn get_user_published_notes(
        &self,
        req: GetUserPublishedNotesRequest,
    ) -> Result<Response<GetUserPublishedNotesResponse>, Status> {
        let notes = self
            .note_repo
            .get_user_published_notes(req.page_size, req.cursor_id, req.user_id)
            .await;

        match notes {
            Ok(notes) => {
                let notes = notes
                    .into_iter()
                    .map(|note| {
                        let note = if note.r#type == NoteType::Normal {
                            Note::NormalNote(NormalNote {
                                id: note.id,
                                title: note.title,
                                content: note.content.unwrap_or_default(),
                                images: Some(ImageList {
                                    images: note.images,
                                }),
                                status: note.status as i32,
                                user_id: note.user_id,
                                created_at: Some(prost_types::Timestamp {
                                    seconds: note.created_at.timestamp(),
                                    nanos: 0,
                                }),
                                updated_at: Some(prost_types::Timestamp {
                                    seconds: note.updated_at.timestamp(),
                                    nanos: 0,
                                }),
                            })
                        } else {
                            Note::VideoNote(VideoNote {
                                id: note.id,
                                title: note.title,
                                content: note.content.unwrap_or_default(),
                                images: Some(ImageList {
                                    images: note.images,
                                }),
                                video: note.video.unwrap_or_default(),
                                status: note.status as i32,
                                user_id: note.user_id,
                                created_at: Some(prost_types::Timestamp {
                                    seconds: note.created_at.timestamp(),
                                    nanos: 0,
                                }),
                                updated_at: Some(prost_types::Timestamp {
                                    seconds: note.updated_at.timestamp(),
                                    nanos: 0,
                                }),
                            })
                        };
                        GetPublishedNoteResponse { note: Some(note) }
                    })
                    .collect();

                let resp = GetUserPublishedNotesResponse { notes };

                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn get_published_notes(
        &self,
        req: GetPublishedNotesRequest,
    ) -> Result<Response<GetPublishedNotesResponse>, Status> {
        let notes = self
            .note_repo
            .get_published_notes(req.page_size, req.cursor)
            .await;

        match notes {
            Ok(notes) => {
                let notes = notes
                    .into_iter()
                    .map(|note| {
                        let note = if note.r#type == NoteType::Normal {
                            Note::NormalNote(NormalNote {
                                id: note.id,
                                title: note.title,
                                content: note.content.unwrap_or_default(),
                                images: Some(ImageList {
                                    images: note.images,
                                }),
                                status: note.status as i32,
                                user_id: note.user_id,
                                created_at: Some(prost_types::Timestamp {
                                    seconds: note.created_at.timestamp(),
                                    nanos: 0,
                                }),
                                updated_at: Some(prost_types::Timestamp {
                                    seconds: note.updated_at.timestamp(),
                                    nanos: 0,
                                }),
                            })
                        } else {
                            Note::VideoNote(VideoNote {
                                id: note.id,
                                title: note.title,
                                content: note.content.unwrap_or_default(),
                                images: Some(ImageList {
                                    images: note.images,
                                }),
                                video: note.video.unwrap_or_default(),
                                status: note.status as i32,
                                user_id: note.user_id,
                                created_at: Some(prost_types::Timestamp {
                                    seconds: note.created_at.timestamp(),
                                    nanos: 0,
                                }),
                                updated_at: Some(prost_types::Timestamp {
                                    seconds: note.updated_at.timestamp(),
                                    nanos: 0,
                                }),
                            })
                        };
                        GetPublishedNoteResponse { note: Some(note) }
                    })
                    .collect();

                let resp = GetPublishedNotesResponse { notes };

                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn get_published_note(
        &self,
        req: GetPublishedNoteRequest,
    ) -> Result<Response<GetPublishedNoteResponse>, Status> {
        let note = self.note_repo.get_published_note_by_id(req.id).await;

        match note {
            Ok(note) => {
                let resp = if note.r#type == NoteType::Normal {
                    GetPublishedNoteResponse {
                        note: Some(Note::NormalNote(NormalNote {
                            id: note.id,
                            title: note.title,
                            content: note.content.unwrap_or_default(),
                            images: Some(ImageList {
                                images: note.images,
                            }),
                            status: note.status as i32,
                            user_id: note.user_id,
                            created_at: Some(prost_types::Timestamp {
                                seconds: note.created_at.timestamp(),
                                nanos: 0,
                            }),
                            updated_at: Some(prost_types::Timestamp {
                                seconds: note.updated_at.timestamp(),
                                nanos: 0,
                            }),
                        })),
                    }
                } else {
                    GetPublishedNoteResponse {
                        note: Some(Note::VideoNote(VideoNote {
                            id: note.id,
                            title: note.title,
                            content: note.content.unwrap_or_default(),
                            images: Some(ImageList {
                                images: note.images,
                            }),
                            video: note.video.unwrap_or_default(),
                            status: note.status as i32,
                            user_id: note.user_id,
                            created_at: Some(prost_types::Timestamp {
                                seconds: note.created_at.timestamp(),
                                nanos: 0,
                            }),
                            updated_at: Some(prost_types::Timestamp {
                                seconds: note.updated_at.timestamp(),
                                nanos: 0,
                            }),
                        })),
                    }
                };
                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn publish_draft_note(
        &self,
        req: PublishDraftNoteRequest,
    ) -> Result<Response<PublishDraftNoteResponse>, Status> {
        let note = self.note_repo.publish_note(req.user_id, req.id).await;

        match note {
            Ok(_) => Ok(Response::new(PublishDraftNoteResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn create_or_update_draft_note(
        &self,
        req: CreateOrUpdateDraftRequest,
    ) -> Result<Response<CreateOrUpdateDraftResponse>, Status> {
        let note = self
            .note_repo
            .upsert_draft_note(
                req.user_id,
                req.id,
                req.title,
                req.content,
                if req.images.is_none() {
                    None
                } else {
                    let list = req.images.unwrap_or_default();
                    Some(list.images)
                },
                req.video,
                if req.status.is_none() {
                    None
                } else {
                    let ret = req.status.unwrap_or_default().try_into();
                    match ret {
                        Ok(status) => Some(status),
                        Err(e) => return Err(Status::invalid_argument(e.to_string())),
                    }
                },
            )
            .await;

        match note {
            Ok(note) => Ok(Response::new(CreateOrUpdateDraftResponse { id: note.id })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn get_user_published_note_ids(
        &self,
        request: GetUserPublishedNoteIdsRequest,
    ) -> Result<Response<GetUserPublishedNoteIdsResponse>, Status> {
        let ids = self
            .note_repo
            .get_user_published_note_ids(request.user_id)
            .await;

        match ids {
            Ok(ids) => Ok(Response::new(GetUserPublishedNoteIdsResponse { ids })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
