use crate::dto::user::User;
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::validators::email;
use async_graphql::{Context, Object, Result};
use im::consumer::user_register_consumer::UserRegisterMessage;
use im::consumer::user_update_consumer::UserUpdateMessage;
use tracing::error;

#[derive(Default)]
pub(crate) struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn follow(&self, ctx: &Context<'_>, user_id: i64) -> Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let follower_id = ctx.data::<i64>()?.clone();

        let _ = state.user_srv.follow_user(follower_id, user_id).await?;

        Ok("".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn unfollow(&self, ctx: &Context<'_>, user_id: i64) -> Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let follower_id = ctx.data::<i64>()?.clone();

        let _ = state.user_srv.unfollow_user(follower_id, user_id).await?;

        Ok("".to_string())
    }

    #[graphql(guard = "AuthGuard")]
    pub async fn profile(
        &self,
        ctx: &Context<'_>,
        fullname: Option<String>,
        avatar: Option<String>,
        bio: Option<String>,
    ) -> Result<String> {
        let state = ctx.data::<AppState>()?.clone();
        let user_id = ctx.data::<i64>()?.clone();

        let _ = state
            .user_srv
            .update_user(user_id, fullname.clone(), avatar.clone(), bio)
            .await?;

        if fullname.is_some() || avatar.is_some() {
            tokio::spawn(async move {
                let data = UserUpdateMessage {
                    user_id,
                    nickname: fullname.unwrap_or_default(),
                    face_url: avatar.unwrap_or_default(),
                };
                let data = serde_json::to_string(&data);
                if let Err(e) = data {
                    error!("failed to serialize user update message: {}", e);
                    return;
                }
                let ret = state.message_queue.produce_message(
                    user_id.to_string().as_bytes(),
                    String::as_bytes(&data.unwrap_or_default()),
                    "UserUpdate",
                );
                if let Err(e) = ret {
                    error!("failed to produce message: {}", e);
                }
            });
        }

        Ok("".to_string())
    }

    pub async fn signin(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        password: String,
    ) -> Result<User> {
        let state = ctx.data::<AppState>()?;

        let user = state.user_srv.verify(email, password).await?;
        ctx.insert_http_header("token", state.jwt_handler.encode(user.id)?);

        Ok(user)
    }

    // 用户注册
    pub async fn signup(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        password: String,
        code: String,
    ) -> Result<User> {
        let state = ctx.data::<AppState>()?.clone();

        let user = state.user_srv.create_user(email, password, code).await?;

        let user_ret = user.clone();
        tokio::spawn(async move {
            let data = UserRegisterMessage {
                user_id: user.id,
                nickname: user.fullname.clone(),
                face_url: user.avatar.clone(),
            };
            let data = serde_json::to_string(&data);
            if let Err(e) = data {
                error!("failed to serialize user register message: {}", e);
                return;
            }
            let ret = state.message_queue.produce_message(
                user.id.to_string().as_bytes(),
                String::as_bytes(&data.unwrap_or_default()),
                "UserRegister",
            );
            if let Err(e) = ret {
                error!("failed to produce message: {}", e);
            }
        });

        Ok(user_ret)
    }

    // 发送邮箱注册验证码
    pub async fn send_register_email_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
    ) -> Result<String> {
        let state = ctx.data::<AppState>()?;

        let _ = state
            .user_srv
            .send_register_email_code(email.clone())
            .await?;

        Ok(format!("Email code has been sent to {}", email))
    }
}
