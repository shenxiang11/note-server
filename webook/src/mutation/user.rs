use crate::model::user::User;
use crate::util::AuthGuard;
use crate::AppState;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub(crate) struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(guard = "AuthGuard")]
    pub async fn profile(
        &self,
        ctx: &Context<'_>,
        fullname: Option<String>,
        avatar: Option<String>,
        bio: Option<String>,
    ) -> Result<User> {
        let state = ctx.data::<AppState>()?;
        let user_id = ctx.data::<i64>()?;

        let user = state
            .user_srv
            .update_profile(*user_id, fullname, avatar, bio)
            .await?;
        Ok(user)
    }

    pub async fn signin(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        password: String,
    ) -> Result<String> {
        let state = ctx.data::<AppState>()?;

        let user = state.user_srv.signin(&email, &password).await?;
        ctx.insert_http_header("token", state.jwt_handler.encode(user.id)?);

        Ok("".to_string())
    }

    // 用户注册
    pub async fn signup(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
        password: String,
        code: String,
    ) -> Result<User> {
        let state = ctx.data::<AppState>()?;

        let user = state.user_srv.signup(&email, &password, &code).await?;

        Ok(user)
    }

    // 发送邮箱注册验证码
    pub async fn send_register_email_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] email: String,
    ) -> Result<String> {
        let state = ctx.data::<AppState>()?;

        let _ = state.user_srv.send_email_code(&email).await?;

        Ok(format!("Email code has been sent to {}", email))
    }
}
