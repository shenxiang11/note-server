use crate::config::AppConfig;
use anyhow::Result;
use jsonwebtoken::{encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct JwtHandler {
    app_config: AppConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
    nbf: u64,
    iat: u64,
    sub: i64,
}

impl Claims {
    fn new(user_id: i64, period_seconds: u64) -> Self {
        let now = get_current_timestamp();
        Claims {
            exp: now + period_seconds,
            nbf: now,
            iat: now,
            sub: user_id,
        }
    }
}

impl JwtHandler {
    pub fn new(app_config: AppConfig) -> Self {
        JwtHandler { app_config }
    }

    pub fn encode(&self, user_id: i64) -> Result<String> {
        let header = Header::new(Algorithm::HS512);
        let claims = Claims::new(user_id, self.app_config.jwt.period_seconds);
        let encode_key = &EncodingKey::from_secret(self.app_config.jwt.sk.as_ref());
        let token = encode(&header, &claims, encode_key)?;
        Ok(token)
    }

    pub fn decode(&self, token: &str) -> Result<i64> {
        let mut validation = jsonwebtoken::Validation::new(Algorithm::HS512);
        validation.leeway = 0;
        let decode_key = &DecodingKey::from_secret(self.app_config.jwt.sk.as_ref());

        let token_data = jsonwebtoken::decode::<Claims>(token, decode_key, &validation)?;

        Ok(token_data.claims.sub)
    }
}
