use crate::model::{Note, NoteStatus, NoteType, PublishedNote};
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

    pub async fn get_published_notes(&self) -> Result<Vec<PublishedNote>> {
        let result: Vec<PublishedNote> = sqlx::query_as(
            r#"
            SELECT * FROM published_notes;
            "#,
        )
        .fetch_all(&self.db_read)
        .await?;

        Ok(result)
    }

    pub async fn upsert(
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

            let r = result.clone();

            if result.status == NoteStatus::Published {
                // 更新 published_notes 表，如果不存在则插入
                sqlx::query(
                    r#"
                    INSERT INTO published_notes (id, user_id, title, content, images, video)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (id)
                    DO UPDATE SET title = $3, content = $4, images = $5, video = $6;
                "#,
                )
                .bind(result.id)
                .bind(result.user_id)
                .bind(result.title)
                .bind(result.content)
                .bind(result.images)
                .bind(result.video)
                .execute(&self.db)
                .await?;
            }
            Ok(r)
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

            let r = result.clone();
            // 发布的话，插入到 published_notes 表
            if result.status == NoteStatus::Published {
                sqlx::query(
                    r#"
                    INSERT INTO published_notes (id, user_id, title, content, images, video)
                    VALUES ($1, $2, $3, $4, $5, $6);
                "#,
                )
                .bind(result.id)
                .bind(result.user_id)
                .bind(result.title)
                .bind(result.content)
                .bind(result.images)
                .bind(result.video)
                .execute(&self.db)
                .await?;
            }

            Ok(r)
        }
    }
}
