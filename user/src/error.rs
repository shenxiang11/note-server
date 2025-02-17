use tonic::Status;

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("password wrong")]
    PasswordWrong,
    #[error("not exists: {0}")]
    NotExists(String),
    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("too frequently: ${0}")]
    TooFrequently(String),
    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("deadpool error: {0}")]
    DeadPoolError(#[from] deadpool::managed::PoolError<redis::RedisError>),
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("user already exists: {0}")]
    UserAlreadyExists(String),
    #[error("email code error")]
    EmailCodeError,
    #[error("internal server error: {0}")]
    InternalServerError(String),
}

impl From<UserServiceError> for Status {
    fn from(err: UserServiceError) -> Self {
        match err {
            _ => Status::internal(err.to_string()),
        }
    }
}
