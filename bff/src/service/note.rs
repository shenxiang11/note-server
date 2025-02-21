use crate::dto::note::{Note, NoteType};
use crate::mutation::note::EditNoteInput;
use crate::util::time::PbTimestamp;
use anyhow::Result;
use note::pb::note::get_published_note_response::Note::{NormalNote, VideoNote};
use note::pb::note::note_service_client::NoteServiceClient;
use note::pb::note::CreateOrUpdateRequest;
use note::pb::note::ImageList;
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
                images: vec![],
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

    pub async fn get_published_notes(&self) -> Result<Vec<Note>> {
        unimplemented!()
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
}
