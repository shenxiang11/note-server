use crate::dto::user::User;
use anyhow::anyhow;
use std::ops::Deref;
use std::sync::Arc;
use tonic::transport::Channel;
use user::pb::user::user_service_client::UserServiceClient;
use user::pb::user::SendRegisterEmailCodeRequest;

#[derive(Clone)]
pub struct UserSrv {
    inner: Arc<UserSrvInner>,
}

impl Deref for UserSrv {
    type Target = UserSrvInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct UserSrvInner {
    client: UserServiceClient<Channel>,
}

impl UserSrv {
    pub fn new(client: UserServiceClient<Channel>) -> Self {
        Self {
            inner: Arc::new(UserSrvInner { client }),
        }
    }

    // 发送邮箱验证码
    // 同一个邮箱，一分钟以内只能发送一次
    // 验证码有效期 10 分钟
    // 验证通过，或者验证三次失败，验证码失效
    pub async fn send_register_email_code(&self, email: String) -> anyhow::Result<()> {
        let mut client = self.client.clone();
        let _ = client
            .send_register_email_code(SendRegisterEmailCodeRequest { email })
            .await?
            .into_inner();

        Ok(())
    }

    // pub async fn signin(&self, email: &str, password: &str) -> anyhow::Result<User, AppError> {
    //     let user: Option<User> = sqlx::query_as(
    //         r#"
    //         SELECT id, serial_number, fullname, email, password_hash, avatar, bio, created_at FROM users
    //         WHERE email = $1
    //         "#,
    //     )
    //         .bind(email)
    //         .fetch_optional(&self.db)
    //         .await?;
    //
    //     // FIXME: 错误处理
    //     match user {
    //         Some(user) => {
    //             if verify_password(password, &user.password_hash)? {
    //                 Ok(user)
    //             } else {
    //                 Err(AppError::ValidationError("password not match".to_string()))
    //             }
    //         }
    //         None => Err(AppError::ValidationError("user not found".to_string())),
    //     }
    // }
    //
    // pub async fn signup(
    //     &self,
    //     email: &str,
    //     password: &str,
    //     code: &str,
    // ) -> anyhow::Result<User, AppError> {
    //     let user = self.get_user_by_email(email).await;
    //     if user.is_ok() {
    //         return Err(AppError::DuplicateKey("email already exists".to_string()));
    //     }
    //
    //     let mut rdb = self.rdb.get().await?;
    //
    //     let lua_script = r#"
    //         local key = KEYS[1]
    //         local cntKey = KEYS[2]
    //
    //         local expectedCode = ARGV[1]
    //
    //         local cnt = tonumber(redis.call("get", cntKey))
    //         local code = redis.call("get", key)
    //
    //         if cnt == nil or cnt <= 0 then
    //             redis.call("del", key)
    //             redis.call("del", cntKey)
    //             return -1
    //         end
    //
    //         if code == expectedCode then
    //             redis.call("del", key)
    //             redis.call("del", cntKey)
    //             return 0
    //         else
    //             redis.call("decr", cntKey)
    //             return -2
    //         end
    //     "#;
    //     let lua_script = Script::new(lua_script);
    //     let key = format!("{}:{}:{}", self.biz, "email_code", email);
    //     let cnt_key = format!("{}:{}", key, "cnt");
    //     let ret: RedisResult<i32> = lua_script
    //         .key(key)
    //         .key(cnt_key)
    //         .arg(code)
    //         .invoke_async(&mut rdb)
    //         .await;
    //
    //     match ret {
    //         Ok(0) => self.create_user(email, password).await,
    //         Ok(-1) | Ok(-2) => Err(AppError::ValidationError("code not match".to_string())),
    //         _ => Err(AppError::InternalServerError),
    //     }
    // }
    //
    // pub async fn update_profile(
    //     &self,
    //     id: i64,
    //     fullname: Option<String>,
    //     avatar: Option<String>,
    //     bio: Option<String>,
    // ) -> anyhow::Result<User, AppError> {
    //     // FIXME: 确认这个是否好
    //     let user: User = sqlx::query_as(
    //         r#"
    //         UPDATE users
    //         SET fullname = COALESCE($1, fullname), avatar = COALESCE($2, avatar), bio = COALESCE($3, bio)
    //         WHERE id = $4
    //         RETURNING *
    //         "#,
    //     ).bind(fullname)
    //         .bind(avatar)
    //         .bind(bio)
    //         .bind(id)
    //         .fetch_one(&self.db).await?;
    //
    //     Ok(user)
    // }
    //
    // pub async fn get_user_by_email(&self, email: &str) -> anyhow::Result<User, AppError> {
    //     let user: Option<User> = sqlx::query_as(
    //         r#"
    //         SELECT id, serial_number, fullname, email, password_hash, avatar, bio, created_at FROM users
    //         WHERE email = $1
    //         "#,
    //     )
    //         .bind(email)
    //         .fetch_optional(&self.db)
    //         .await?;
    //
    //     match user {
    //         Some(user) => Ok(user),
    //         None => Err(AppError::NotFound("user not found".to_string())),
    //     }
    // }

    pub async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let mut client = self.client.clone();
        let resp = client
            .get_user_by_id(user::pb::user::GetUserByIdRequest { id })
            .await?
            .into_inner();

        match resp.user {
            Some(user) => Ok(user.into()),
            None => Err(anyhow!("user not found")),
        }
    }

    // async fn create_user(&self, email: &str, password: &str) -> anyhow::Result<User, AppError> {
    //     let serial_number = self.gen_serial_no().await?;
    //     let fullname = self.gen_fullname();
    //     let password_hash = hash_password(password)?;
    //
    //     let ret = sqlx::query_as(
    //         r#"
    //         INSERT INTO users (serial_number, fullname, email, password_hash)
    //         VALUES ($1, $2, $3, $4)
    //         RETURNING *
    //         "#,
    //     )
    //     .bind(&serial_number)
    //     .bind(&fullname)
    //     .bind(email)
    //     .bind(&password_hash)
    //     .fetch_one(&self.db)
    //     .await;
    //
    //     match ret {
    //         Ok(user) => {
    //             let user: User = user;
    //             Ok(user)
    //         }
    //         Err(e) => {
    //             if let Error::Database(db_err) = e {
    //                 if let code = db_err.code().ok_or(AppError::InternalServerError)? {
    //                     if code == "23505" {
    //                         return Err(AppError::DuplicateKey(db_err.message().to_string()));
    //                     }
    //                 }
    //             }
    //             // FIXME: 不想返回这个错误，该是什么错误返回什么错误
    //             Err(AppError::InternalServerError)
    //         }
    //     }
    // }
}
