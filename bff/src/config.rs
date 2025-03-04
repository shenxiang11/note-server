use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppConfig {
    inner: Arc<AppConfigInner>,
}

impl Deref for AppConfig {
    type Target = Arc<AppConfigInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigInner {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub kafka: KafkaConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub postgres_url: String,
    pub postgres_url_read: String,
    pub redis_url: String,
    pub request_id_header: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtConfig {
    pub pk: String,
    pub sk: String,
    pub period_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        let env = std::env::var("NOTE_ENV").unwrap_or_else(|_| "test".to_string());

        #[cfg(not(test))]
        let config_data = if env == "test" {
            include_str!("../config.test.toml")
        } else {
            include_str!("../config.prod.toml")
        };
        #[cfg(test)]
        let config_data = include_str!("../config.test.toml");

        Self {
            inner: Arc::new(toml::from_str(&config_data).expect("Failed to load config")),
        }
    }
}
