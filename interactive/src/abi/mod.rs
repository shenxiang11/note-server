use crate::model::{CountBiz, UserCollectsBiz, UserLikesBiz};
use crate::pb::{
    BatchGetCountRequest, BatchGetCountResponse, BatchGetIsLikedRequest, BatchGetIsLikedResponse,
    BizIdsAndUserIdsAndIsLiked, CollectRequest, CollectResponse, GetCountRequest, GetCountResponse,
    LikeRequest, LikeResponse, SaveCountRequest, SaveCountResponse, UncollectRequest,
    UncollectResponse, UnlikeRequest, UnlikeResponse,
};
use crate::InteractiveSrv;
use tonic::{Response, Status};

impl InteractiveSrv {
    pub async fn batch_get_is_liked(
        &self,
        req: BatchGetIsLikedRequest,
    ) -> Result<Response<BatchGetIsLikedResponse>, Status> {
        let ret: Result<UserLikesBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };
        let biz_ids_and_user_ids = req.query.iter().map(|q| (q.biz_id, q.user_id)).collect();
        let ret = self
            .interactive_repo
            .batch_get_is_liked(biz, biz_ids_and_user_ids)
            .await;

        match ret {
            Ok(is_liked) => {
                let resp = BatchGetIsLikedResponse {
                    results: is_liked
                        .iter()
                        .map(|(k, v)| BizIdsAndUserIdsAndIsLiked {
                            biz_id: k.0,
                            user_id: k.1,
                            is_liked: *v,
                        })
                        .collect(),
                };
                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn save_count(
        &self,
        req: SaveCountRequest,
    ) -> Result<Response<SaveCountResponse>, Status> {
        let ret: Result<CountBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .interactive_repo
            .save_count(biz, req.biz_id, req.n)
            .await;

        match ret {
            Ok(_) => Ok(Response::new(SaveCountResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn get_count(
        &self,
        req: GetCountRequest,
    ) -> Result<Response<GetCountResponse>, Status> {
        let ret: Result<CountBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self.interactive_repo.get_count(biz, req.biz_id).await;

        match ret {
            Ok(count) => Ok(Response::new(GetCountResponse { count })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn like(&self, req: LikeRequest) -> Result<Response<LikeResponse>, Status> {
        let ret: Result<UserLikesBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .interactive_repo
            .save_like(biz, req.biz_id, req.user_id)
            .await;

        match ret {
            Ok(_) => Ok(Response::new(LikeResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn unlike(&self, req: UnlikeRequest) -> Result<Response<UnlikeResponse>, Status> {
        let ret: Result<UserLikesBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .interactive_repo
            .cancel_like(biz, req.biz_id, req.user_id)
            .await;

        match ret {
            Ok(_) => Ok(Response::new(UnlikeResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn collect(&self, req: CollectRequest) -> Result<Response<CollectResponse>, Status> {
        let ret: Result<UserCollectsBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .interactive_repo
            .save_collect(biz, req.biz_id, req.user_id)
            .await;

        match ret {
            Ok(_) => Ok(Response::new(CollectResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn uncollect(
        &self,
        req: UncollectRequest,
    ) -> Result<Response<UncollectResponse>, Status> {
        let ret: Result<UserCollectsBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        let ret = self
            .interactive_repo
            .cancel_collect(biz, req.biz_id, req.user_id)
            .await;

        match ret {
            Ok(_) => Ok(Response::new(UncollectResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    pub async fn batch_get_count(
        &self,
        req: BatchGetCountRequest,
    ) -> Result<Response<BatchGetCountResponse>, Status> {
        let ret: Result<CountBiz, _> = req.biz.try_into();
        let biz = match ret {
            Ok(biz) => biz,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };
        let mut ret = self
            .interactive_repo
            .batch_get_count(biz, req.biz_ids)
            .await;

        match ret {
            Ok(counts) => {
                let resp = BatchGetCountResponse { counts };
                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
