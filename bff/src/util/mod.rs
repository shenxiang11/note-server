pub mod jwt_handler;
pub mod message_queue;
pub mod time;

use crate::app_error::AppError;
use async_graphql::{Context, Guard, Result, ResultExt};

pub(crate) struct AuthGuard;

impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let user_id = ctx.data::<i64>();
        if user_id.is_err() {
            return Err(AppError::Unauthorized).extend();
        }

        Ok(())
    }
}
