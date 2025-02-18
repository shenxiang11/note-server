use crate::error::UserServiceError;
use crate::model::User;
use anyhow::Result;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Local;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use redis::{AsyncCommands, RedisResult, Script};
use sqlx::{Error, PgPool};

pub struct UserRepo {
    biz: String,
    db: PgPool,
    db_read: PgPool,
    rdb: deadpool_redis::Pool,
}

impl UserRepo {
    pub fn new(db: PgPool, db_read: PgPool, rdb: deadpool_redis::Pool) -> Self {
        Self {
            biz: "user".to_string(),
            db,
            db_read,
            rdb,
        }
    }

    // 发送邮件应该是属于消息服务
    // 发送邮箱验证码
    // 同一个邮箱，一分钟以内只能发送一次
    // 验证码有效期 10 分钟
    // 验证通过，或者验证三次失败，验证码失效
    pub async fn send_email_code(&self, email: &str) -> Result<String, UserServiceError> {
        let user = self.get_user_by_email(email).await;
        if user.is_ok() {
            return Err(UserServiceError::EmailAlreadyExists);
        }

        let code = rand::random::<u32>() % 100000;
        let code = format!("{:0>6}", code);

        let mut rdb = self.rdb.get().await?;
        // 使用 lua 脚本保证原子性
        let lua_script = r#"
            local key = KEYS[1]
            local cntKey = KEYS[2]
            local val = ARGV[1]
            local ttl = tonumber(redis.call("ttl", key))

            if ttl == - 1 then
                return -2
            elseif ttl == -2 or ttl < 540 then
                redis.call("set", key, val)
                redis.call("expire", key, 600)
                redis.call("set", cntKey, 3)
                redis.call("expire", cntKey, 600)
                return 0
            else
                return -1
            end
        "#;
        let lua_script = Script::new(lua_script);
        let key = format!("{}:{}:{}", self.biz, "email_code", email);
        let cnt_key = format!("{}:{}", key, "cnt");
        let ret = lua_script
            .key(key)
            .key(cnt_key)
            .arg(code.clone())
            .invoke_async(&mut rdb)
            .await;

        match ret {
            Ok(0) => {
                // 不想真的发，现在控制台看验证码吧
                // send_email_code(email, &code).await?;
                Ok(code)
            }
            Ok(-1) => Err(UserServiceError::TooFrequently(
                "send email code too frequently".to_string(),
            )),
            // key 永远不会过期？那就是有问题，不能让你发
            Ok(-2) | _ => Err(UserServiceError::InternalServerError(
                "please check your key, it should hava a ttl.".to_string(),
            )),
            _ => Err(UserServiceError::InternalServerError("Unknown".to_string())),
        }
    }

    pub async fn verify(&self, email: &str, password: &str) -> Result<User, UserServiceError> {
        let user: Option<User> = sqlx::query_as(
            r#"
            SELECT id, serial_number, fullname, email, password_hash, avatar, bio, created_at FROM users
            WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_optional(&self.db)
            .await?;

        match user {
            Some(user) => {
                if verify_password(password, &user.password_hash)? {
                    Ok(user)
                } else {
                    Err(UserServiceError::PasswordWrong)
                }
            }
            None => Err(UserServiceError::NotExists("user not found".to_string())),
        }
    }

    pub async fn create_user(
        &self,
        email: &str,
        password: &str,
        code: &str,
    ) -> Result<User, UserServiceError> {
        let user = self.get_user_by_email(email).await;
        if user.is_ok() {
            return Err(UserServiceError::EmailAlreadyExists);
        }

        let mut rdb = self.rdb.get().await?;

        let lua_script = r#"
            local key = KEYS[1]
            local cntKey = KEYS[2]

            local expectedCode = ARGV[1]

            local cnt = tonumber(redis.call("get", cntKey))
            local code = redis.call("get", key)

            if cnt == nil or cnt <= 0 then
                redis.call("del", key)
                redis.call("del", cntKey)
                return -1
            end

            if code == expectedCode then
                redis.call("del", key)
                redis.call("del", cntKey)
                return 0
            else
                redis.call("decr", cntKey)
                return -2
            end
        "#;
        let lua_script = Script::new(lua_script);
        let key = format!("{}:{}:{}", self.biz, "email_code", email);
        let cnt_key = format!("{}:{}", key, "cnt");
        let ret: RedisResult<i32> = lua_script
            .key(key)
            .key(cnt_key)
            .arg(code)
            .invoke_async(&mut rdb)
            .await;

        match ret {
            Ok(0) => self.create_user_inner(email, password).await,
            Ok(-1) | Ok(2) => Err(UserServiceError::EmailCodeError),
            _ => Err(UserServiceError::InternalServerError("Unknown".to_string())),
        }
    }

    pub async fn update(
        &self,
        id: i64,
        fullname: Option<String>,
        avatar: Option<String>,
        bio: Option<String>,
    ) -> Result<User, UserServiceError> {
        let user: User = sqlx::query_as(
            r#"
            UPDATE users
            SET fullname = COALESCE($1, fullname), avatar = COALESCE($2, avatar), bio = COALESCE($3, bio)
            WHERE id = $4
            RETURNING *
            "#,
        ).bind(fullname)
            .bind(avatar)
            .bind(bio)
            .bind(id)
            .fetch_one(&self.db).await?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, UserServiceError> {
        let user: Option<User> = sqlx::query_as(
            r#"
            SELECT id, serial_number, fullname, email, password_hash, avatar, bio, created_at FROM users
            WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_optional(&self.db)
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserServiceError::NotExists("user not found".to_string())),
        }
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<User, UserServiceError> {
        let user: Option<User> = sqlx::query_as(
            r#"
            SELECT id, serial_number, fullname, email, password_hash, avatar, bio, created_at FROM users
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.db)
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(UserServiceError::NotExists("user not found".to_string())),
        }
    }

    async fn create_user_inner(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, UserServiceError> {
        let serial_number = self.gen_serial_no().await?;
        let fullname = self.gen_fullname();
        let password_hash = hash_password(password)?;

        let ret = sqlx::query_as(
            r#"
            INSERT INTO users (serial_number, fullname, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&serial_number)
        .bind(&fullname)
        .bind(email)
        .bind(&password_hash)
        .fetch_one(&self.db)
        .await;

        match ret {
            Ok(user) => {
                let user: User = user;
                Ok(user)
            }
            Err(e) => {
                let msg = e.to_string();
                if let Error::Database(db_err) = e {
                    if let code = db_err
                        .code()
                        .ok_or(UserServiceError::InternalServerError("".to_string()))?
                    {
                        if code == "23505" {
                            return Err(UserServiceError::UserAlreadyExists(
                                db_err.message().to_string(),
                            ));
                        }
                    }
                }
                Err(UserServiceError::InternalServerError(msg))
            }
        }
    }

    fn gen_fullname(&self) -> String {
        let random_chars = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(3)
            .map(char::from)
            .collect::<String>();
        format!("小黄人{}", random_chars)
    }

    async fn gen_serial_no(&self) -> Result<String, UserServiceError> {
        let date_prefix = Local::now().format("%Y%m%d%H").to_string();
        let mut rdb = self.rdb.get().await?;

        let rkey = format!("{}:{}:{}", self.biz, "serial_no", date_prefix);
        let no: i32 = rdb.incr(&rkey, 1).await?;

        if no == 1 {
            let now = Local::now();
            let expire_time = now + chrono::Duration::hours(1);
            let expire_at = expire_time.to_utc().timestamp();

            rdb.expire_at(&rkey, expire_at).await?;
        }

        // 其实超过 9999 也没关系，只是流水号会变长 1 位，每小时不超过 9999 个注册就不会有此问题
        let serial_no = format!("{}{:0>4}", date_prefix, no);
        Ok(serial_no)
    }
}

// 使用邮箱发送验证码
async fn send_email_code(email: &str, code: &str) -> Result<()> {
    let from = "863461783@qq.com".parse().unwrap();
    let to = email.parse().unwrap();
    let header = "text/html; charset=utf8".parse().unwrap();

    let message = MessageBuilder::new()
        .from(from)
        .to(to)
        .subject("Your Email Verification Code.")
        .header(lettre::message::header::ContentType::from(header))
        .body(format!("<h1>Your verification code is: {}</h1>", code))?;

    let creds = Credentials::new(
        "863461783@qq.com".to_string(),
        "ucqzmsgjeuqjbccf".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.qq.com")?
        .credentials(creds)
        .build();

    mailer.send(&message)?;

    Ok(())
}

fn hash_password(password: &str) -> Result<String, UserServiceError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, UserServiceError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}
