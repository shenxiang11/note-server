use crate::dto::note::{Note, NoteType};
use crate::mutation::note::EditNoteInput;
use crate::util::time::PbTimestamp;
use anyhow::Result;
use note::pb::note::get_published_note_response::Note::{NormalNote, VideoNote};
use note::pb::note::note_service_client::NoteServiceClient;
use note::pb::note::ImageList;
use note::pb::note::{CreateOrUpdateRequest, GetUserPublishedNoteIdsRequest};
use std::ops::Deref;
use std::sync::Arc;
use tonic::transport::Channel;

pub struct NoteSrv {
    inner: Arc<NoteSrvInner>,
}

impl Deref for NoteSrv {
    type Target = NoteSrvInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct NoteSrvInner {
    client: NoteServiceClient<Channel>,
}

impl NoteSrv {
    pub fn new(client: NoteServiceClient<Channel>) -> Self {
        Self {
            inner: Arc::new(NoteSrvInner { client }),
        }
    }

    pub async fn get_published_note_by_id(&self, id: i64) -> Result<Note> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(note::pb::note::GetPublishedNoteRequest { id });
        let response = client.get_published_note(request).await?;
        let response = response.into_inner();
        let note = response.note;

        match note {
            Some(NormalNote(note)) => Ok(Note {
                id: note.id,
                title: note.title,
                content: note.content,
                images: note.images.unwrap_or_default().images,
                video: "".to_string(),
                status: note.status.into(),
                user_id: note.user_id,
                created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                r#type: NoteType::Normal,
            }),
            Some(VideoNote(note)) => Ok(Note {
                id: note.id,
                title: note.title,
                content: note.content,
                images: note.images.unwrap_or_default().images,
                video: note.video,
                status: note.status.into(),
                user_id: note.user_id,
                created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                r#type: NoteType::Video,
            }),
            _ => unimplemented!(),
        }
    }

    pub async fn get_user_published_notes(
        &self,
        page_size: i64,
        cursor: Option<i64>,
        user_id: i64,
    ) -> Result<Vec<Note>> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(note::pb::note::GetUserPublishedNotesRequest {
            page_size,
            cursor_id: cursor,
            user_id,
        });
        let response = client.get_user_published_notes(request).await?;
        let response = response.into_inner();
        let notes = response.notes;
        let notes = notes
            .into_iter()
            .map(|note| match note.note {
                Some(NormalNote(note)) => Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    images: note.images.unwrap_or_default().images,
                    video: "".to_string(),
                    status: note.status.into(),
                    user_id: note.user_id,
                    created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                    r#type: NoteType::Normal,
                },
                Some(VideoNote(note)) => Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    images: note.images.unwrap_or_default().images,
                    video: note.video,
                    status: note.status.into(),
                    user_id: note.user_id,
                    created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                    r#type: NoteType::Video,
                },
                _ => unimplemented!(),
            })
            .collect();
        Ok(notes)
    }

    pub async fn get_published_notes(
        &self,
        page_size: i64,
        cursor: Option<i64>,
    ) -> Result<Vec<Note>> {
        let mut client = self.client.clone();
        let request =
            tonic::Request::new(note::pb::note::GetPublishedNotesRequest { page_size, cursor });
        let response = client.get_published_notes(request).await?;
        let response = response.into_inner();
        let notes = response.notes;
        let notes = notes
            .into_iter()
            .map(|note| match note.note {
                Some(NormalNote(note)) => Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    images: note.images.unwrap_or_default().images,
                    video: "".to_string(),
                    status: note.status.into(),
                    user_id: note.user_id,
                    created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                    r#type: NoteType::Normal,
                },
                Some(VideoNote(note)) => Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    images: note.images.unwrap_or_default().images,
                    video: note.video,
                    status: note.status.into(),
                    user_id: note.user_id,
                    created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                    r#type: NoteType::Video,
                },
                _ => unimplemented!(),
            })
            .collect();
        Ok(notes)
    }

    pub async fn create_or_update(
        &self,
        user_id: i64,
        note_id: Option<i64>,
        input: EditNoteInput,
    ) -> Result<()> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(CreateOrUpdateRequest {
            user_id,
            id: note_id,
            title: input.title,
            content: input.content,
            images: if let Some(images) = input.images {
                Some(ImageList { images })
            } else {
                None
            },
            video: input.video,
            status: if let Some(status) = input.status {
                Some(status as i32)
            } else {
                None
            },
        });
        client.create_or_update(request).await?;
        Ok(())
    }

    pub async fn get_published_note_ids_by_user(&self, user_id: i64) -> Result<Vec<i64>> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(GetUserPublishedNoteIdsRequest { user_id });
        let response = client.get_user_published_note_ids(request).await?;
        let response = response.into_inner();
        Ok(response.ids)
    }

    pub async fn batch_get_published_notes(&self, ids: Vec<i64>) -> Result<Vec<Note>> {
        let mut client = self.client.clone();
        let request = tonic::Request::new(note::pb::note::BatchGetPublishedNotesRequest { ids });
        let response = client.batch_get_published_notes(request).await?;
        let response = response.into_inner();
        let notes = response.notes;
        let notes = notes
            .into_iter()
            .map(|note| match note.note {
                Some(NormalNote(note)) => Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    images: note.images.unwrap_or_default().images,
                    video: "".to_string(),
                    status: note.status.into(),
                    user_id: note.user_id,
                    created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                    r#type: NoteType::Normal,
                },
                Some(VideoNote(note)) => Note {
                    id: note.id,
                    title: note.title,
                    content: note.content,
                    images: note.images.unwrap_or_default().images,
                    video: note.video,
                    status: note.status.into(),
                    user_id: note.user_id,
                    created_at: PbTimestamp::from(note.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(note.updated_at.unwrap_or_default()).into(),
                    r#type: NoteType::Video,
                },
                _ => unimplemented!(),
            })
            .collect();
        Ok(notes)
    }
}
