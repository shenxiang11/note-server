use crate::model::{Note, NoteStatus, NoteType, PublishedNote, PublishedNoteStatus};
use anyhow::Result;
use sqlx::PgPool;

pub struct NoteRepo {
    biz: String,
    db: PgPool,
    db_read: PgPool,
}

impl NoteRepo {
    pub fn new(db: PgPool, db_read: PgPool) -> Self {
        Self {
            biz: "note".to_string(),
            db,
            db_read,
        }
    }

    pub async fn get_published_note_by_id(&self, id: i64) -> Result<PublishedNote> {
        let result: PublishedNote = sqlx::query_as(
            r#"
            SELECT * FROM published_notes WHERE id = $1;
            "#,
        )
        .bind(id)
        .fetch_one(&self.db_read)
        .await?;

        Ok(result)
    }

    pub async fn batch_get_published_notes(&self, ids: Vec<i64>) -> Result<Vec<PublishedNote>> {
        let result: Vec<PublishedNote> = sqlx::query_as(
            r#"
            SELECT * FROM published_notes WHERE id = ANY($1) ORDER BY id DESC;
            "#,
        )
        .bind(ids)
        .fetch_all(&self.db_read)
        .await?;

        Ok(result)
    }

    pub async fn get_user_published_notes(
        &self,
        page_size: i64,
        last_id: Option<i64>,
        user_id: i64,
    ) -> Result<Vec<PublishedNote>> {
        let cursor = last_id.unwrap_or_else(|| i64::MAX);

        let result: Vec<PublishedNote> = sqlx::query_as(
            r#"
            SELECT * FROM published_notes WHERE id < $1 AND user_id = $2 ORDER BY id DESC LIMIT $3;
            "#,
        )
        .bind(cursor)
        .bind(user_id)
        .bind(page_size)
        .fetch_all(&self.db_read)
        .await?;

        Ok(result)
    }

    pub async fn get_published_notes(
        &self,
        page_size: i64,
        last_id: Option<i64>,
    ) -> Result<Vec<PublishedNote>> {
        let cursor = last_id.unwrap_or_else(|| i64::MAX);

        let result: Vec<PublishedNote> = sqlx::query_as(
            r#"
            SELECT * FROM published_notes WHERE id < $1 ORDER BY id DESC LIMIT $2;
            "#,
        )
        .bind(cursor)
        .bind(page_size)
        .fetch_all(&self.db_read)
        .await?;

        Ok(result)
    }

    pub async fn get_user_published_note_ids(&self, user_id: i64) -> Result<Vec<i64>> {
        let result: Vec<i64> = sqlx::query_scalar(
            r#"
            SELECT id FROM published_notes WHERE user_id = $1;
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.db_read)
        .await?;

        Ok(result)
    }

    pub async fn upsert_draft_note(
        &self,
        user_id: i64,
        id: Option<i64>,
        title: Option<String>,
        content: Option<String>,
        images: Option<Vec<String>>,
        video: Option<String>,
        status: Option<NoteStatus>,
    ) -> Result<Note> {
        let note_type = if video.is_some() {
            NoteType::Video
        } else {
            NoteType::Normal
        };

        if let Some(id) = id {
            let result: Note = sqlx::query_as(
                r#"
                UPDATE notes
                SET title = COALESCE($1, title), content = COALESCE($2, content), images = COALESCE($3, images), video = COALESCE($4, video), status = COALESCE($7, status), updated_at = now()
                WHERE id = $5 AND user_id = $6
                RETURNING *;
            "#,
            )
                .bind(title)
                .bind(content)
                .bind(images)
                .bind(video)
                .bind(id)
                .bind(user_id)
                .bind(status)
                .fetch_one(&self.db)
                .await?;

            Ok(result)
        } else {
            if title.is_none() {
                return Err(anyhow::anyhow!("title is required"));
            }

            let result: Note = sqlx::query_as(
                r#"
                INSERT INTO notes (user_id, type, title, content, images, video, status)
                VALUES ($1, $2, $3, COALESCE($4, null), $5, $6, COALESCE($7, 'draft'))
                RETURNING *;
            "#,
            )
            .bind(user_id)
            .bind(note_type)
            .bind(title)
            .bind(content)
            .bind(images.unwrap_or_default())
            .bind(video)
            .bind(status)
            .fetch_one(&self.db)
            .await?;

            Ok(result)
        }
    }

    pub async fn publish_note(&self, user_id: i64, id: i64) -> Result<PublishedNote> {
        let result: Note = sqlx::query_as(
            r#"
            UPDATE notes
            SET status = 'published', updated_at = now()
            WHERE id = $1 AND user_id = $2
            RETURNING *;
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        let published_note: PublishedNote = sqlx::query_as(
            r#"
            INSERT INTO published_notes (id, user_id, type, status, title, content, images, video)
            VALUES ($1, $2, $3, $4, $5, COALESCE($6, null), $7, $8)
            ON CONFLICT (id) DO UPDATE SET status = $4, title = $5, content = COALESCE($6, null), images = $7, video = $8, updated_at = now()
            RETURNING *;
            "#,
        )
            .bind(result.id)
            .bind(result.user_id)
            .bind(result.r#type)
            .bind(PublishedNoteStatus::Published)
            .bind(result.title)
            .bind(result.content)
            .bind(result.images)
            .bind(result.video)
            .fetch_one(&self.db)
            .await?;

        Ok(published_note)
    }
}
