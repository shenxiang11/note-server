use crate::pb::comment::comment_service_server::CommentService;
use crate::pb::comment::{
    BatchGetRepliesRequest, BatchGetRepliesResponse, DeleteCommentRequest, DeleteCommentResponse,
    GetCommentsRequest, GetCommentsResponse, GetMoreCommentsRequest, GetMoreCommentsResponse,
    SaveCommentRequest, SaveCommentResponse,
};
use crate::repository::CommentRepo;
use tonic::{async_trait, Request, Response, Status};

pub mod abi;
pub mod config;
pub mod model;
pub mod pb;
pub mod repository;

pub struct CommentSrv {
    comment_repo: CommentRepo,
}

impl CommentSrv {
    pub fn new(comment_repo: CommentRepo) -> Self {
        Self { comment_repo }
    }
}

#[async_trait]
impl CommentService for CommentSrv {
    async fn batch_get_replies(
        &self,
        request: Request<BatchGetRepliesRequest>,
    ) -> Result<Response<BatchGetRepliesResponse>, Status> {
        self.batch_get_replies(request.into_inner()).await
    }

    async fn save_comment(
        &self,
        request: Request<SaveCommentRequest>,
    ) -> Result<Response<SaveCommentResponse>, Status> {
        self.save_comment(request.into_inner()).await
    }

    async fn get_comments(
        &self,
        request: Request<GetCommentsRequest>,
    ) -> Result<Response<GetCommentsResponse>, Status> {
        self.get_comments(request.into_inner()).await
    }

    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>,
    ) -> Result<Response<DeleteCommentResponse>, Status> {
        todo!()
    }

    async fn get_more_comments(
        &self,
        request: Request<GetMoreCommentsRequest>,
    ) -> Result<Response<GetMoreCommentsResponse>, Status> {
        todo!()
    }
}

fn comment_to_pb(comment: model::Comment) -> pb::comment::Comment {
    pb::comment::Comment {
        id: comment.id,
        user_id: comment.user_id,
        biz: comment.biz as i32,
        biz_id: comment.biz_id,
        root_id: comment.root_id,
        parent_id: comment.parent_id,
        content: comment.content.clone(),
        created_at: Some(prost_types::Timestamp {
            seconds: comment.created_at.timestamp(),
            nanos: 0,
        }),
        updated_at: Some(prost_types::Timestamp {
            seconds: comment.updated_at.timestamp(),
            nanos: 0,
        }),
    }
}
