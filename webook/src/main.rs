use anyhow::Result;
use std::net::SocketAddr;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use webook::config::AppConfig;
use webook::{start_server, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_config = AppConfig::load();

    // let config = RustlsConfig::from_pem_file(
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("self_signed_certs")
    //         .join("cert.pem"),
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("self_signed_certs")
    //         .join("key.pem"),
    // )
    // .await?;

    let app_state = AppState::new(app_config.clone()).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], app_config.server.port));
    start_server(app_state, addr).await?;

    Ok(())
}
