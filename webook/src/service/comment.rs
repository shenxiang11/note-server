use crate::dto::comment::Comment;
use crate::util::time::PbTimestamp;
use anyhow::Result;
use comment::pb::comment::comment_service_client::CommentServiceClient;
use comment::pb::comment::{
    BatchGetRepliesRequest, CommentBiz, GetCommentsRequest, SaveCommentRequest,
};
use std::collections::HashMap;
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

    pub async fn batch_get_replies(&self, biz_ids: Vec<i64>) -> Result<HashMap<i64, Vec<Comment>>> {
        let mut client = self.client.clone();
        let ret = client
            .batch_get_replies(BatchGetRepliesRequest { ids: biz_ids })
            .await?
            .into_inner();

        let mut result: HashMap<i64, Vec<Comment>> = HashMap::new();
        for (biz_id, comments) in ret.replies.into_iter() {
            let comments = comments.comments.into_iter().map(|c| c.into()).collect();
            result.insert(biz_id, comments);
        }

        Ok(result)
    }

    pub async fn find_comments(
        &self,
        comment_biz: CommentBiz,
        biz_id: i64,
        min_id: i64,
        limit: i64,
    ) -> Result<Vec<Comment>> {
        let mut client = self.client.clone();
        let ret = client
            .get_comments(GetCommentsRequest {
                biz: comment_biz as i32,
                biz_id,
                min_id,
                limit,
            })
            .await?
            .into_inner();

        let comments = ret.comments.into_iter().map(|c| c.into()).collect();

        Ok(comments)
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
                    content: c.content,
                    created_at: PbTimestamp::from(c.created_at.unwrap_or_default()).into(),
                    updated_at: PbTimestamp::from(c.updated_at.unwrap_or_default()).into(),
                })
            }
            None => Err(anyhow::anyhow!("Failed to create comment")),
        }
    }
}
