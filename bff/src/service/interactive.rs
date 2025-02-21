use anyhow::Result;
use interactive::pb::interactive_service_client::InteractiveServiceClient;
use interactive::pb::{CountBiz, GetCountRequest, LikeRequest, UnlikeRequest, UserLikesBiz};
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
}
