use crate::model::{CountBiz, Counter, NoteReadMessage, UserHistoryBiz, UserLikesBiz};
use anyhow::Result;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct InteractiveRepo {
    inner: Arc<InteractiveRepoInner>,
}

impl Deref for InteractiveRepo {
    type Target = InteractiveRepoInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct InteractiveRepoInner {
    biz: String,
    db: PgPool,
    db_read: PgPool,
}

impl InteractiveRepo {
    pub fn new(db: PgPool, db_read: PgPool) -> Self {
        Self {
            inner: Arc::new(InteractiveRepoInner {
                biz: "interactive".to_string(),
                db,
                db_read,
            }),
        }
    }

    pub async fn save_count(&self, biz: CountBiz, biz_id: i64, n: i64) -> Result<()> {
        let _ = sqlx::query(
            r#"
            INSERT INTO counters (biz, biz_id, count)
            VALUES ($1, $2, $3)
            ON CONFLICT (biz, biz_id)
            DO UPDATE SET count = counters.count + $3, updated_at = now();
            "#,
        )
        .bind(biz)
        .bind(biz_id)
        .bind(n)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_count(&self, biz: CountBiz, biz_id: i64) -> Result<i64> {
        let ret: Result<Counter, _> = sqlx::query_as(
            r#"
            SELECT *
            FROM counters
            WHERE biz = $1 AND biz_id = $2;
            "#,
        )
        .bind(biz)
        .bind(biz_id)
        .fetch_one(&self.db_read)
        .await;

        match ret {
            Ok(counter) => Ok(counter.count),
            Err(e) => Ok(0),
        }
    }

    pub async fn save_histories(
        &self,
        biz: UserHistoryBiz,
        message: Vec<NoteReadMessage>,
    ) -> Result<()> {
        for message in message.iter() {
            let _ = sqlx::query(
                r#"
                INSERT INTO user_histories (biz, biz_id, user_id, created_at)
                VALUES ($1, $2, $3, now())
                ON CONFLICT (biz, biz_id, user_id)
                DO UPDATE SET updated_at = now();
                "#,
            )
            .bind(biz)
            .bind(message.biz_id)
            .bind(message.user_id)
            .execute(&self.db)
            .await;
        }

        Ok(())
    }

    pub async fn save_like(&self, biz: UserLikesBiz, biz_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_likes (biz, biz_id, user_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (biz, biz_id, user_id)
            DO UPDATE SET deleted_at = NULL, updated_at = now();
            "#,
        )
        .bind(biz)
        .bind(biz_id)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn cancel_like(&self, biz: UserLikesBiz, biz_id: i64, user_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_likes
            SET deleted_at = now()
            WHERE biz = $1 AND biz_id = $2 AND user_id = $3;
            "#,
        )
        .bind(biz)
        .bind(biz_id)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
