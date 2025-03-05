use deadpool::Runtime;
use deadpool_redis::Config;
use sqlx::PgPool;
use std::net::SocketAddr;
use tonic::transport::Server;
use user::config::AppConfig;
use user::pb::user::user_service_server::UserServiceServer;
use user::repository::UserRepo;
use user::UserSrv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_config = AppConfig::load();

    let addr = SocketAddr::from(([0, 0, 0, 0], app_config.server.port));

    let db = PgPool::connect(app_config.server.postgres_url.as_str())
        .await
        .expect("Failed to create pg pool");

    let db_read = PgPool::connect(app_config.server.postgres_url_read.as_str())
        .await
        .expect("Failed to create pg pool");

    let redis_cfg = Config::from_url(app_config.server.redis_url.as_str());

    let rdb = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create redis pool");

    let user_repo = UserRepo::new(db, db_read, rdb);
    let interactive_srv = UserSrv::new(user_repo);

    Server::builder()
        .add_service(UserServiceServer::new(interactive_srv))
        .serve(addr)
        .await?;

    Ok(())
}
