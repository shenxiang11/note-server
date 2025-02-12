use crate::model::CommentBiz;
use crate::pb::comment;
use crate::pb::comment::{
    GetCommentsRequest, GetCommentsResponse, ListOfComment, SaveCommentRequest,
};
use crate::{comment_to_pb, CommentSrv};
use comment::{BatchGetRepliesResponse, SaveCommentResponse};
use tonic::{Response, Status};

impl CommentSrv {
    pub async fn batch_get_replies(
        &self,
        req: comment::BatchGetRepliesRequest,
    ) -> Result<Response<BatchGetRepliesResponse>, Status> {
        let ret = self.comment_repo.batch_get_replies(req.ids).await;

        match ret {
            Ok(replies) => {
                let replies = replies
                    .into_iter()
                    .map(|row| {
                        let (id, replies) = row;
                        let replies = ListOfComment {
                            comments: replies.into_iter().map(|c| comment_to_pb(c)).collect(),
                        };
                        (id, replies)
                    })
                    .collect();
                Ok(Response::new(BatchGetRepliesResponse { replies }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn save_comment(
        &self,
        req: SaveCommentRequest,
    ) -> Result<Response<SaveCommentResponse>, Status> {
        let ret: Result<CommentBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .comment_repo
            .create(
                req.user_id,
                biz,
                req.biz_id,
                req.root_id,
                req.parent_id,
                req.content,
            )
            .await;

        match ret {
            Ok(comment) => Ok(Response::new(SaveCommentResponse {
                comment: Some(comment_to_pb(comment)),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn get_comments(
        &self,
        req: GetCommentsRequest,
    ) -> Result<Response<GetCommentsResponse>, Status> {
        let ret: Result<CommentBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .comment_repo
            .find_by_biz(biz, req.biz_id, req.min_id, req.limit)
            .await;

        match ret {
            Ok(comments) => {
                let comments = comments.into_iter().map(|c| comment_to_pb(c)).collect();
                Ok(Response::new(GetCommentsResponse { comments }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
