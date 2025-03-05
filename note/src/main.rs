use anyhow::Result;
use note::config::AppConfig;
use note::pb::note::note_service_server::NoteServiceServer;
use note::repository::NoteRepo;
use note::NoteSrv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let app_config = AppConfig::load();

    let addr = SocketAddr::from(([0, 0, 0, 0], app_config.server.port));

    let db = PgPool::connect(app_config.server.postgres_url.as_str())
        .await
        .expect("Failed to create pg pool");

    let db_read = PgPool::connect(app_config.server.postgres_url_read.as_str())
        .await
        .expect("Failed to create pg pool");

    let note_repo = NoteRepo::new(db, db_read);

    let note_srv = NoteSrv::new(note_repo);

    Server::builder()
        .add_service(NoteServiceServer::new(note_srv))
        .serve(addr)
        .await?;

    Ok(())
}
