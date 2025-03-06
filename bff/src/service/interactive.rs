use anyhow::Result;
use interactive::pb::interactive_service_client::InteractiveServiceClient;
use interactive::pb::{
    CollectRequest, CountBiz, GetCountRequest, GetUserCollectedNoteIdsRequest,
    GetUserLikedNoteIdsRequest, LikeRequest, UncollectRequest, UnlikeRequest, UserCollectsBiz,
    UserLikesBiz,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct InteractiveSrv {
    inner: Arc<InteractiveSrvInner>,
}

impl Deref for InteractiveSrv {
    type Target = InteractiveSrvInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct InteractiveSrvInner {
    client: InteractiveServiceClient<Channel>,
}

impl InteractiveSrv {
    pub fn new(client: InteractiveServiceClient<Channel>) -> Self {
        Self {
            inner: Arc::new(InteractiveSrvInner { client }),
        }
    }

    pub async fn collect(&self, user_id: i64, biz: UserCollectsBiz, biz_id: i64) -> Result<()> {
        let mut client = self.client.clone();
        client
            .collect(CollectRequest {
                biz: biz as i32,
                user_id,
                biz_id,
            })
            .await?;

        Ok(())
    }

    pub async fn uncollect(&self, user_id: i64, biz: UserCollectsBiz, biz_id: i64) -> Result<()> {
        let mut client = self.client.clone();
        client
            .uncollect(UncollectRequest {
                biz: biz as i32,
                user_id,
                biz_id,
            })
            .await?;

        Ok(())
    }

    pub async fn like(&self, user_id: i64, biz: UserLikesBiz, biz_id: i64) -> Result<()> {
        let mut client = self.client.clone();
        client
            .like(LikeRequest {
                biz: biz as i32,
                user_id,
                biz_id,
            })
            .await?;

        Ok(())
    }

    pub async fn unlike(&self, user_id: i64, biz: UserLikesBiz, biz_id: i64) -> Result<()> {
        let mut client = self.client.clone();
        client
            .unlike(UnlikeRequest {
                biz: biz as i32,
                user_id,
                biz_id,
            })
            .await?;

        Ok(())
    }

    pub async fn batch_get_collected(
        &self,
        biz: UserCollectsBiz,
        biz_ids_and_user_ids: Vec<(i64, i64)>,
    ) -> Result<HashMap<(i64, i64), bool>> {
        let mut client = self.client.clone();
        let resp = client
            .batch_get_is_collected(interactive::pb::BatchGetIsCollectedRequest {
                biz: biz as i32,
                query: biz_ids_and_user_ids
                    .into_iter()
                    .map(|(biz_id, user_id)| interactive::pb::BizIdsAndUserIds { biz_id, user_id })
                    .collect(),
            })
            .await?
            .into_inner();

        Ok(resp
            .results
            .iter()
            .map(|row| ((row.biz_id, row.user_id), row.is_collected))
            .collect())
    }

    pub async fn batch_get_liked(
        &self,
        biz: UserLikesBiz,
        biz_ids_and_user_ids: Vec<(i64, i64)>,
    ) -> Result<HashMap<(i64, i64), bool>> {
        let mut client = self.client.clone();
        let resp = client
            .batch_get_is_liked(interactive::pb::BatchGetIsLikedRequest {
                biz: biz as i32,
                query: biz_ids_and_user_ids
                    .into_iter()
                    .map(|(biz_id, user_id)| interactive::pb::BizIdsAndUserIds { biz_id, user_id })
                    .collect(),
            })
            .await?
            .into_inner();

        Ok(resp
            .results
            .iter()
            .map(|row| ((row.biz_id, row.user_id), row.is_liked))
            .collect())
    }

    pub async fn batch_get_count(
        &self,
        biz: CountBiz,
        biz_ids: Vec<i64>,
    ) -> Result<HashMap<i64, i64>> {
        let mut client = self.client.clone();
        let resp = client
            .batch_get_count(interactive::pb::BatchGetCountRequest {
                biz: biz as i32,
                biz_ids,
            })
            .await?
            .into_inner();
        Ok(resp.counts)
    }

    pub async fn get_count(&self, biz: CountBiz, biz_id: i64) -> Result<i64> {
        let mut client = self.client.clone();
        let resp = client
            .get_count(GetCountRequest {
                biz: biz as i32,
                biz_id,
            })
            .await?
            .into_inner();

        Ok(resp.count)
    }

    pub async fn get_user_liked_note_ids(
        &self,
        user_id: i64,
        page_size: i64,
        cursor_id: Option<i64>,
    ) -> Result<Vec<i64>> {
        let mut client = self.client.clone();
        let resp = client
            .get_user_liked_note_ids(GetUserLikedNoteIdsRequest {
                user_id,
                page_size,
                cursor_id,
            })
            .await?
            .into_inner();

        Ok(resp.ids)
    }

    pub async fn get_user_collected_note_ids(
        &self,
        user_id: i64,
        page_size: i64,
        cursor_id: Option<i64>,
    ) -> Result<Vec<i64>> {
        let mut client = self.client.clone();
        let resp = client
            .get_user_collected_note_ids(GetUserCollectedNoteIdsRequest {
                user_id,
                page_size,
                cursor_id,
            })
            .await?
            .into_inner();

        Ok(resp.ids)
    }
}
