use crate::model::{Comment, CommentBiz};
use anyhow::Result;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct CommentRepo {
    inner: Arc<CommentRepoInner>,
}

impl Deref for CommentRepo {
    type Target = CommentRepoInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct CommentRepoInner {
    biz: String,
    db: PgPool,
    db_read: PgPool,
}

impl CommentRepo {
    pub fn new(db: PgPool, db_read: PgPool) -> Self {
        Self {
            inner: Arc::new(CommentRepoInner {
                biz: "comment".to_string(),
                db,
                db_read,
            }),
        }
    }

    pub async fn create(
        &self,
        user_id: i64,
        biz: CommentBiz,
        biz_id: i64,
        root_id: Option<i64>,
        parent_id: Option<i64>,
        content: String,
    ) -> Result<Comment> {
        let comment: Comment = sqlx::query_as(
            r#"
            INSERT INTO comments (user_id, biz, biz_id, root_id, parent_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *;
            "#,
        )
        .bind(user_id)
        .bind(biz)
        .bind(biz_id)
        .bind(root_id)
        .bind(parent_id)
        .bind(content)
        .fetch_one(&self.db)
        .await?;

        Ok(comment)
    }

    pub async fn find_by_biz(
        &self,
        biz: CommentBiz,
        biz_id: i64,
        min_id: i64,
        limit: i64,
    ) -> Result<Vec<Comment>> {
        let ret: Vec<Comment> = sqlx::query_as(
            r#"
            SELECT *
            FROM comments
            WHERE biz = $1 AND biz_id = $2 AND  id < $3
            ORDER BY id DESC
            LIMIT $4;
            "#,
        )
        .bind(biz as CommentBiz)
        .bind(biz_id)
        .bind(min_id)
        .bind(limit)
        .fetch_all(&self.db_read)
        .await?;

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::model::CommentBiz;

    #[tokio::test]
    async fn test_create() {
        let config = AppConfig::load();
        let pool = PgPool::connect(&config.server.postgres_url).await.unwrap();
        let pool_read = PgPool::connect(&config.server.postgres_url_read)
            .await
            .unwrap();
        let repo = CommentRepo::new(pool, pool_read);

        let user_id = 1;
        let biz = CommentBiz::Note;
        let biz_id = 2;
        let root_id = None;
        let parent_id = None;
        let content = "君君太美了！！气质超好👍👍".to_string();
        let ret = repo
            .create(user_id, biz, biz_id, root_id, parent_id, content)
            .await;

        assert!(ret.is_ok());
    }

    #[tokio::test]
    async fn test_find_by_biz() {
        let config = AppConfig::load();
        let pool = PgPool::connect(&config.server.postgres_url).await.unwrap();
        let pool_read = PgPool::connect(&config.server.postgres_url_read)
            .await
            .unwrap();
        let repo = CommentRepo::new(pool, pool_read);

        let biz = CommentBiz::Note;
        let biz_id = 2;
        let min_id = i64::MAX;
        let limit = 2;
        let ret = repo.find_by_biz(biz, biz_id, min_id, limit).await;
        assert!(ret.is_ok());

        println!("{:?}", ret);

        if ret.is_ok() {
            let comments = ret.unwrap();
            let min_id = comments.last().unwrap().id;
            let ret = repo.find_by_biz(biz, biz_id, min_id, limit).await;
            assert!(ret.is_ok());
            println!("{:?}", ret);
        }
    }
}
