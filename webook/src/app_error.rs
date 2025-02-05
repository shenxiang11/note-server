use async_graphql::{Error, ErrorExtensions};

#[derive(thiserror::Error, Debug)]
pub(crate) enum AppError {
    #[error("stmp error: {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),

    #[error("lettre error: {0}")]
    LettreError(#[from] lettre::error::Error),

    #[error("deadpool error: {0}")]
    DeadPoolError(#[from] deadpool::managed::PoolError<redis::RedisError>),

    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("duplicate key: {0}")]
    DuplicateKey(String),

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("internal server error")]
    InternalServerError,
}

const ERROR_CODE_NAME: &str = "code";

impl ErrorExtensions for AppError {
    fn extend(&self) -> Error {
        // FIXME: 目前没有发现全部 result 自动调用 extend 的方法，在前端关心某个具体错误的地方先手动调用一下，比如 401 403
        Error::new(format!("{}", self)).extend_with(|_, e| {
            let code = match self {
                AppError::Unauthorized => 401,
                _ => 500,
            };

            e.set(ERROR_CODE_NAME, code);
        })
    }
}
