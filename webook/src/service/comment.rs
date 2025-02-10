use crate::util::time::PbTimestamp;
use anyhow::Result;
use comment::model;
use comment::model::Comment;
use comment::pb::comment::comment_service_client::CommentServiceClient;
use comment::pb::comment::{CommentBiz, SaveCommentRequest};
use std::ops::Deref;
use std::sync::Arc;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct CommentSrv {
    inner: Arc<CommentSrvInner>,
}

impl Deref for CommentSrv {
    type Target = CommentSrvInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct CommentSrvInner {
    client: CommentServiceClient<Channel>,
}

impl CommentSrv {
    pub fn new(client: CommentServiceClient<Channel>) -> Self {
        Self {
            inner: Arc::new(CommentSrvInner { client }),
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
        let mut client = self.client.clone();
        let ret = client
            .save_comment(SaveCommentRequest {
                user_id,
                biz: biz as i32,
                biz_id,
                root_id,
                parent_id,
                content,
            })
            .await?
            .into_inner();

        match ret.comment {
            Some(comment) => {
                let c = comment;
                Ok(Comment {
                    id: c.id,
                    user_id: c.user_id,
                    biz: model::CommentBiz::Comment,
                    biz_id: c.biz_id,
                    root_id: c.root_id,
                    parent_id: c.parent_id,
                    content: c.content,
                    created_at: PbTimestamp::from(c.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(c.updated_at.unwrap_or_default()).into(),
                })
            }
            None => Err(anyhow::anyhow!("Failed to create comment")),
        }
    }
}
