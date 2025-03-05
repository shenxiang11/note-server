use anyhow::Result;
use comment::config::AppConfig;
use comment::pb::comment::comment_service_server::CommentServiceServer;
use comment::repository::CommentRepo;
use comment::CommentSrv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tonic::transport::Server;

mod model;

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

    let comment_repo = CommentRepo::new(db, db_read);

    let comment_srv = CommentSrv::new(comment_repo.clone());

    Server::builder()
        .add_service(CommentServiceServer::new(comment_srv))
        .serve(addr)
        .await?;

    Ok(())
}
