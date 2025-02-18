use crate::AppState;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub(crate) struct UserMutation;

#[Object]
impl UserMutation {
    // #[graphql(guard = "AuthGuard")]
    // pub async fn profile(
    //     &self,
    //     ctx: &Context<'_>,
    //     fullname: Option<String>,
    //     avatar: Option<String>,
    //     bio: Option<String>,
    // ) -> Result<User> {
    //     let state = ctx.data::<AppState>()?.clone();
    //     let user_id = ctx.data::<i64>()?;
    //
    //     let user_ret = state
    //         .user_srv
    //         .update_profile(*user_id, fullname.clone(), avatar.clone(), bio)
    //         .await?;
    //
    //     if fullname.is_some() || avatar.is_some() {
    //         let user = user_ret.clone();
    //         tokio::spawn(async move {
    //             let data = UserUpdateMessage {
    //                 user_id: user.id,
    //                 nickname: user.fullname.clone(),
    //                 face_url: user.avatar.clone(),
    //             };
    //             let data = serde_json::to_string(&data);
    //             if let Err(e) = data {
    //                 error!("failed to serialize user update message: {}", e);
    //                 return;
    //             }
    //             let ret = state.message_queue.produce_message(
    //                 user.id.to_string().as_bytes(),
    //                 String::as_bytes(&data.unwrap_or_default()),
    //                 "UserUpdate",
    //             );
    //             if let Err(e) = ret {
    //                 error!("failed to produce message: {}", e);
    //             }
    //         });
    //     }
    //
    //     Ok(user_ret)
    // }

    // pub async fn signin(
    //     &self,
    //     ctx: &Context<'_>,
    //     #[graphql(validator(email))] email: String,
    //     password: String,
    // ) -> Result<String> {
    //     let state = ctx.data::<AppState>()?;
    //
    //     let user = state.user_srv.signin(&email, &password).await?;
    //     ctx.insert_http_header("token", state.jwt_handler.encode(user.id)?);
    //
    //     Ok("".to_string())
    // }
    //
    // // 用户注册
    // pub async fn signup(
    //     &self,
    //     ctx: &Context<'_>,
    //     #[graphql(validator(email))] email: String,
    //     password: String,
    //     code: String,
    // ) -> Result<User> {
    //     let state = ctx.data::<AppState>()?.clone();
    //
    //     let user = state.user_srv.signup(&email, &password, &code).await?;
    //
    //     let user_ret = user.clone();
    //     tokio::spawn(async move {
    //         let data = UserRegisterMessage {
    //             user_id: user.id,
    //             nickname: user.fullname.clone(),
    //             face_url: user.avatar.clone(),
    //         };
    //         let data = serde_json::to_string(&data);
    //         if let Err(e) = data {
    //             error!("failed to serialize user register message: {}", e);
    //             return;
    //         }
    //         let ret = state.message_queue.produce_message(
    //             user.id.to_string().as_bytes(),
    //             String::as_bytes(&data.unwrap_or_default()),
    //             "UserRegister",
    //         );
    //         if let Err(e) = ret {
    //             error!("failed to produce message: {}", e);
    //         }
    //     });
    //
    //     Ok(user_ret)
    // }

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
