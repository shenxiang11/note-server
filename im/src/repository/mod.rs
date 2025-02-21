use crate::consumer::user_register_consumer::UserRegisterMessage;
use crate::consumer::user_update_consumer::UserUpdateMessage;
use anyhow::Result;
use chrono::Utc;
use redis::AsyncCommands;
use serde::Deserialize;
use std::collections::HashMap;

pub struct IMRepo {
    api_address: String,
    secret: String,
    user_id: String,
    rdb: deadpool_redis::Pool,
}

impl IMRepo {
    pub fn new(api_address: String, rdb: deadpool_redis::Pool) -> Self {
        Self {
            api_address,
            rdb,
            secret: "openIM123".to_string(),
            user_id: "imAdmin".to_string(),
        }
    }

    pub async fn update_or_create_user(
        &self,
        user_id: i64,
        nickname: String,
        face_url: String,
    ) -> Result<()> {
        let check = self.check_user(user_id).await?;
        println!("check user: {}", check);

        if check {
            self.update_user(UserUpdateMessage {
                user_id,
                nickname,
                face_url,
            })
            .await?;
        } else {
            self.create_users(vec![UserRegisterMessage {
                user_id,
                nickname,
                face_url,
            }])
            .await?;
        }
        Ok(())
    }

    pub async fn update_user(&self, user: UserUpdateMessage) -> Result<()> {
        let token = self.get_token().await?;
        let client = reqwest::Client::new();
        let url = format!("{}/user/update_user_info_ex", self.api_address);
        let mut params = HashMap::new();
        let mut userInfo = HashMap::new();
        userInfo.insert("userID", user.user_id.to_string());
        userInfo.insert("nickname", user.nickname.clone());
        userInfo.insert("faceUrl", user.face_url.clone());
        params.insert("userInfo", userInfo);
        let resp = client
            .post(&url)
            .header("OperationID", Utc::now().timestamp_millis().to_string())
            .header("token", token)
            .json(&params)
            .send()
            .await?;

        let resp = resp.json::<IMResponse<EmptyData>>().await?;
        if resp.err_code != 0 {
            return Err(anyhow::anyhow!("sync user failed: {}", resp.err_msg));
        }
        Ok(())
    }

    pub async fn create_users(&self, users: Vec<UserRegisterMessage>) -> Result<()> {
        let token = self.get_token().await?;
        let client = reqwest::Client::new();
        let url = format!("{}/user/user_register", self.api_address);
        let mut params = HashMap::new();
        params.insert(
            "users",
            users
                .iter()
                .map(|u| {
                    let mut map = HashMap::new();
                    map.insert("userID", u.user_id.to_string());
                    map.insert("nickname", u.nickname.clone());
                    map.insert("faceUrl", u.face_url.clone());
                    map
                })
                .collect::<Vec<_>>(),
        );
        let resp = client
            .post(&url)
            .header("OperationID", Utc::now().timestamp_millis().to_string())
            .header("token", token)
            .json(&params)
            .send()
            .await?;
        let resp = resp.json::<IMResponse<EmptyData>>().await?;

        if resp.err_code != 0 {
            return Err(anyhow::anyhow!("sync user failed: {}", resp.err_msg));
        }

        Ok(())
    }

    async fn check_user(&self, check_id: i64) -> Result<bool> {
        let token = self.get_token().await?;
        let client = reqwest::Client::new();
        let url = format!("{}/user/account_check", self.api_address);
        let mut params = HashMap::new();
        params.insert("checkUserIDs", vec![check_id.to_string()]);
        let resp = client
            .post(&url)
            .header("OperationID", Utc::now().timestamp_millis().to_string())
            .header("token", token)
            .json(&params)
            .send()
            .await?;
        let resp = resp.json::<IMResponse<CheckUserData>>().await?;

        if resp.err_code != 0 {
            return Err(anyhow::anyhow!("sync user failed: {}", resp.err_msg));
        }

        match resp.data {
            Some(data) => {
                if data.results.is_empty() {
                    return Ok(false);
                }
                Ok(data.results[0].account_status == 1)
            }
            None => Err(anyhow::anyhow!("sync user failed: {}", resp.err_msg)),
        }
    }

    async fn get_token(&self) -> Result<String> {
        let mut conn = self.rdb.get().await?;
        let token: Option<String> = conn.get("im:admin::token").await?;

        if let Some(token) = token {
            return Ok(token);
        }

        let client = reqwest::Client::new();
        let url = format!("{}/auth/get_admin_token", self.api_address);
        let mut params = HashMap::new();
        params.insert("secret", self.secret.clone());
        params.insert("userID", self.user_id.clone());

        let resp = client
            .post(&url)
            .header("OperationID", Utc::now().timestamp_millis().to_string())
            .json(&params)
            .send()
            .await?;
        let resp = resp.json::<IMResponse<AdminData>>().await?;

        match resp.data {
            Some(data) => {
                let token = data.token.clone();
                conn.set_ex("im:admin::token", data.token, data.expire_time_seconds)
                    .await?;
                Ok(token)
            }
            None => Err(anyhow::anyhow!("get token failed: {}", resp.err_msg)),
        }
    }
}

#[derive(Deserialize, Debug)]
struct AdminData {
    token: String,
    #[serde(rename = "expireTimeSeconds")]
    expire_time_seconds: u64,
}

#[derive(Deserialize, Debug)]
struct CheckUserData {
    results: Vec<CheckUserDataItem>,
}

#[derive(Deserialize, Debug)]
struct CheckUserDataItem {
    #[serde(rename = "userID")]
    user_id: String,
    #[serde(rename = "accountStatus")]
    account_status: i8,
}

#[derive(Deserialize, Debug)]
struct IMResponse<T> {
    #[serde(rename = "errCode")]
    err_code: i32,
    #[serde(rename = "errMsg")]
    err_msg: String,
    #[serde(rename = "errDlt")]
    err_dlt: String,
    data: Option<T>,
}

#[derive(Deserialize, Debug)]
struct EmptyData;
