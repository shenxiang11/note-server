use crate::model::{CountBiz, UserLikesBiz};
use crate::pb::{
    GetCountRequest, GetCountResponse, LikeRequest, LikeResponse, SaveCountRequest,
    SaveCountResponse, UnlikeRequest, UnlikeResponse,
};
use crate::InteractiveSrv;
use tonic::{Response, Status};

impl InteractiveSrv {
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
}
