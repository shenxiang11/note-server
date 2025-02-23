use anyhow::Result;
use interactive::config::AppConfig;
use interactive::consumer::note_collect_consumer::NoteCollectConsumer;
use interactive::consumer::note_comment_consumer::NoteCommentConsumer;
use interactive::consumer::note_like_consumer::NoteLikeConsumer;
use interactive::consumer::note_read_consumer::NoteReadConsumer;
use interactive::pb::interactive_service_server::InteractiveServiceServer;
use interactive::repository::InteractiveRepo;
use interactive::InteractiveSrv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();

    let app_config = AppConfig::load();

    let addr = SocketAddr::from(([127, 0, 0, 1], app_config.server.port));

    let db = PgPool::connect(app_config.server.postgres_url.as_str())
        .await
        .expect("Failed to create pg pool");

    let db_read = PgPool::connect(app_config.server.postgres_url_read.as_str())
        .await
        .expect("Failed to create pg pool");

    let interactive_repo = InteractiveRepo::new(db.clone(), db_read.clone());
    let interactive_srv = InteractiveSrv::new(interactive_repo.clone());

    let cfg = app_config.clone();
    tokio::spawn(async move {
        let note_read_consumer = NoteReadConsumer::new(cfg.kafka.brokers.clone(), interactive_repo);
        if let Err(e) = note_read_consumer.consume() {
            debug!("failed to consume note read message: {}", e);
        }
    });

    let interactive_repo = InteractiveRepo::new(db.clone(), db_read.clone());
    let cfg = app_config.clone();
    tokio::spawn(async move {
        let note_comment_consumer =
            NoteCommentConsumer::new(cfg.kafka.brokers.clone(), interactive_repo.clone());
        if let Err(e) = note_comment_consumer.consume() {
            debug!("failed to consume note comment message: {}", e);
        }
    });

    let interactive_repo = InteractiveRepo::new(db.clone(), db_read.clone());
    let cfg = app_config.clone();
    tokio::spawn(async move {
        let note_like_consumer =
            NoteLikeConsumer::new(cfg.kafka.brokers.clone(), interactive_repo.clone());
        if let Err(e) = note_like_consumer.consume() {
            debug!("failed to consume note comment message: {}", e);
        }
    });

    let interactive_repo = InteractiveRepo::new(db, db_read);
    let cfg = app_config.clone();
    tokio::spawn(async move {
        let note_collect_consumer =
            NoteCollectConsumer::new(cfg.kafka.brokers.clone(), interactive_repo.clone());
        if let Err(e) = note_collect_consumer.consume() {
            debug!("failed to consume note comment message: {}", e);
        }
    });

    Server::builder()
        .add_service(InteractiveServiceServer::new(interactive_srv))
        .serve(addr)
        .await?;

    Ok(())
}
