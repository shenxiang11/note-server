use crate::model::NoteType;
use crate::pb::note::{
    get_published_note_response, CreateOrUpdateRequest, CreateOrUpdateResponse,
    GetPublishedNoteRequest, GetPublishedNoteResponse, ImageList, NormalNote, VideoNote,
};
use crate::NoteSrv;
use get_published_note_response::Note;
use tonic::{Response, Status};

impl NoteSrv {
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

    pub async fn create_or_update(
        &self,
        req: CreateOrUpdateRequest,
    ) -> Result<Response<CreateOrUpdateResponse>, Status> {
        let note = self
            .note_repo
            .upsert(
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
            Ok(_) => Ok(Response::new(CreateOrUpdateResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
