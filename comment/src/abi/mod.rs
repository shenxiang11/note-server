use crate::model::CommentBiz;
use crate::pb::comment;
use crate::pb::comment::SaveCommentRequest;
use crate::CommentSrv;
use comment::SaveCommentResponse;
use tonic::{Response, Status};

impl CommentSrv {
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
            Ok(comment) => {
                let c = comment;
                let comment = comment::Comment {
                    id: c.id,
                    user_id: c.user_id,
                    biz: c.biz as i32,
                    biz_id: c.biz_id,
                    root_id: c.root_id,
                    parent_id: c.parent_id,
                    content: c.content,
                    created_at: Some(prost_types::Timestamp {
                        seconds: c.created_at.timestamp(),
                        nanos: 0,
                    }),
                    updated_at: Some(prost_types::Timestamp {
                        seconds: c.updated_at.timestamp(),
                        nanos: 0,
                    }),
                };
                Ok(Response::new(SaveCommentResponse {
                    comment: Some(comment),
                }))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
