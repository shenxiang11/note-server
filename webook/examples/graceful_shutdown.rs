//! Run with
//!
//! ```not_rust
//! cargo run -p example-graceful-shutdown
//! kill or ctrl-c
//! ```

use std::time::Duration;

use axum::routing::post;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::time::sleep;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Enable tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    // Create a regular axum app.
    let app = Router::new()
        .route("/slow", post(|| sleep(Duration::from_secs(5))))
        .route("/forever", get(std::future::pending::<()>))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

// 在浏览器里同时发出请求，如果是同样的 get 请求，不能做到每个请求都优雅退出，是浏览器的问题，不是 axum grace_ful 的问题

// for (let i = 0; i < 6; i++) {
// fetch(`http://localhost:3000/slow?id=${i}`, {
// "headers": {
// "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
// "accept-language": "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7",
// "cache-control": "max-age=0",
// "sec-ch-ua": "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
// "sec-ch-ua-mobile": "?0",
// "sec-ch-ua-platform": "\"macOS\"",
// "sec-fetch-dest": "document",
// "sec-fetch-mode": "navigate",
// "sec-fetch-site": "none",
// "sec-fetch-user": "?1",
// "upgrade-insecure-requests": "1"
// },
// "referrerPolicy": "strict-origin-when-cross-origin",
// "body": null,
// "method": "GET",
// "mode": "cors",
// "credentials": "include"
// });
// }
