use crate::model::{CountBiz, Counter, NoteReadMessage, UserHistoryBiz, UserLikesBiz};
use anyhow::Result;
use sqlx::{Database, PgPool, Row};
use std::collections::HashMap;
use std::hash::Hash;
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

    pub async fn batch_get_is_liked(
        &self,
        biz: UserLikesBiz,
        biz_ids_and_user_ids: Vec<(i64, i64)>,
    ) -> Result<HashMap<(i64, i64), bool>> {
        let mut is_liked = HashMap::new();

        let biz_ids_and_user_ids = biz_ids_and_user_ids
            .iter()
            .map(|(biz_id, user_id)| format!("({}, {})", biz_id, user_id))
            .collect::<Vec<String>>()
            .join(", ");

        let query_str = format!(
            r#"
            SELECT *
            FROM user_likes
            WHERE biz = $1 AND (biz_id, user_id) IN ({}) AND deleted_at IS NULL;
            "#,
            biz_ids_and_user_ids
        );

        let _ = sqlx::query(query_str.as_str())
            .bind(biz)
            .bind(biz_ids_and_user_ids)
            .fetch_all(&self.db_read)
            .await?
            .iter()
            .for_each(|row| {
                let biz_id: i64 = row.get("biz_id");
                let user_id: i64 = row.get("user_id");
                is_liked.insert((biz_id, user_id), true);
            });

        Ok(is_liked)
    }

    pub async fn batch_get_count(
        &self,
        biz: CountBiz,
        biz_ids: Vec<i64>,
    ) -> Result<HashMap<i64, i64>> {
        let counts: HashMap<i64, i64> = sqlx::query(
            r#"
            SELECT *
            FROM counters
            WHERE biz = $1 AND biz_id = ANY($2);
            "#,
        )
        .bind(biz)
        .bind(&biz_ids)
        .fetch_all(&self.db_read)
        .await?
        .iter()
        .map(|row| {
            let biz_id: i64 = row.get("biz_id");
            let count: i64 = row.get("count");
            (biz_id, count)
        })
        .collect();

        Ok(counts)
    }

    pub async fn save_count(&self, biz: CountBiz, biz_id: i64, n: i64) -> Result<()> {
        println!(
            "save_count: biz: {:?}, biz_id: {:?}, n: {:?}",
            biz, biz_id, n
        );
        let ret = sqlx::query(
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
        .await;

        if let Err(e) = ret {
            println!("save_count failed: {}", e);
        }

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
        let ret = sqlx::query(
            r#"
            INSERT INTO user_likes (biz, biz_id, user_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (biz, biz_id, user_id)
            DO UPDATE SET deleted_at = NULL, updated_at = now()
            WHERE user_likes.deleted_at IS NOT NULL;
            "#,
        )
        .bind(biz)
        .bind(biz_id)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        if ret.rows_affected() == 0 {
            return Err(anyhow::anyhow!("save_like failed"));
        }

        Ok(())
    }

    pub async fn cancel_like(&self, biz: UserLikesBiz, biz_id: i64, user_id: i64) -> Result<()> {
        let ret = sqlx::query(
            r#"
            UPDATE user_likes
            SET deleted_at = now()
            WHERE biz = $1 AND biz_id = $2 AND user_id = $3 AND deleted_at IS NULL;
            "#,
        )
        .bind(biz)
        .bind(biz_id)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        if ret.rows_affected() == 0 {
            return Err(anyhow::anyhow!("cancel_like failed"));
        }

        Ok(())
    }
}
