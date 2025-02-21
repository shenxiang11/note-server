use anyhow::Result;
use deadpool::Runtime;
use deadpool_redis::Config;
use im::config::AppConfig;
use im::consumer::user_register_consumer::UserRegisterConsumer;
use im::consumer::user_update_consumer::{UserUpdateConsumer, UserUpdateMessage};
use im::repository::IMRepo;
use tokio::sync::mpsc;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(2);

    let app_config = AppConfig::load();
    let cfg = app_config.clone();

    let redis_cfg = Config::from_url(app_config.server.redis_url.as_str());

    let rdb = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create redis pool");

    let im_repo = IMRepo::new("http://192.168.1.7:10002".to_string(), rdb.clone());
    let tx1 = tx.clone();
    tokio::spawn(async move {
        let consumer = UserRegisterConsumer::new(cfg.kafka.brokers.clone(), im_repo);
        if let Err(e) = consumer.consume().await {
            debug!("failed to consume note read message: {}", e);
        }
        tx1.send("done").await.expect("TODO: panic message");
    });
    let cfg = app_config.clone();
    let im_repo = IMRepo::new("http://192.168.1.7:10002".to_string(), rdb);
    tokio::spawn(async move {
        let consumer = UserUpdateConsumer::new(cfg.kafka.brokers.clone(), im_repo);
        if let Err(e) = consumer.consume().await {
            debug!("failed to consume note read message: {}", e);
        }
        tx.send("done").await.expect("TODO: panic message");
    });

    let msg = rx.recv().await.expect("TODO: panic message");
    debug!("received message: {}", msg);

    Ok(())
}
