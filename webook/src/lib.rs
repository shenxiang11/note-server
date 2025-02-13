use crate::config::AppConfig;
use crate::data_loader::comment_replies_loader::CommentRepliesLoader;
use crate::data_loader::note_comments_count_loader::NoteCommentsCountLoader;
use crate::data_loader::note_views_loader::NoteViewsLoader;
use crate::data_loader::replies_count_loader::RepliesCountLoader;
use crate::mutation::MutationRoot;
use crate::query::QueryRoot;
use crate::service::comment::CommentSrv;
use crate::service::interactive::InteractiveSrv;
use crate::service::note::NoteSrv;
use crate::service::user::UserSrv;
use crate::util::jwt_handler::JwtHandler;
use crate::util::message_queue::MessageQueue;
use anyhow::Result;
use async_graphql::dataloader::DataLoader;
use async_graphql::extensions::ExtensionFactory;
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderName, Request};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{response, Extension, Router};
use comment::pb::comment::comment_service_client::CommentServiceClient;
use deadpool::Runtime;
use deadpool_redis::Config;
use interactive::pb::interactive_service_client::InteractiveServiceClient;
use note::pb::note::note_service_client::NoteServiceClient;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::request_id::{
    MakeRequestId, MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

mod app_error;
pub mod config;
mod data_loader;
pub mod dto;
mod model;
mod mutation;
mod query;
mod service;
mod util;

const REQUEST_ID_HEADER: &str = "x-request-id";
const GRAPHQL_ENDPOINT: &str = "/graphql";

pub async fn start_server(
    app_state: AppState,
    addr: SocketAddr,
    // config: RustlsConfig,
) -> Result<()> {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(DataLoader::new(
        CommentRepliesLoader::new(app_state.comment_srv.clone()),
        tokio::spawn,
    ))
    .data(DataLoader::new(
        NoteViewsLoader::new(app_state.interactive_srv.clone()),
        tokio::spawn,
    ))
    .data(DataLoader::new(
        RepliesCountLoader::new(app_state.comment_srv.clone()),
        tokio::spawn,
    ))
    .data(DataLoader::new(
        NoteCommentsCountLoader::new(app_state.comment_srv.clone()),
        tokio::spawn,
    ))
    .data(app_state.clone())
    .finish();

    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);
    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id));

    let app = Router::new()
        .route("/graphiql", get(graphiql))
        .route(GRAPHQL_ENDPOINT, post(graphql_handler))
        .layer(middleware)
        .layer(Extension(app_state.clone()))
        .with_state(schema);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    // let mut server = axum_server::bind_rustls(addr, config);
    //
    // server.http_builder().http2().enable_connect_protocol();
    //
    // let handle = Handle::new();
    // let shutdown_handle = handle.clone();
    // tokio::spawn(async move {
    //     shutdown_signal().await;
    //     // 给一些时间让请求处理完
    //     sleep(std::time::Duration::from_secs(3));
    //     shutdown_handle.shutdown();
    // });
    //
    // server.handle(handle).serve(app.into_make_service()).await?;

    Ok(())
}

pub async fn get_user_id_from_bearer_token(state: AppState, str: Option<&str>) -> Option<i64> {
    if let Some(token) = str {
        if token.starts_with("Bearer ") {
            let token = token.trim_start_matches("Bearer ");
            let user_id = state.jwt_handler.decode(token);

            match user_id {
                Ok(user_id) => Some(user_id),
                Err(_) => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

async fn graphql_handler(
    State(schema): State<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    let token = headers
        .get("Authorization")
        .map(|v| v.to_str().unwrap_or_default());

    let user_id = get_user_id_from_bearer_token(state, token).await;
    if let Some(user_id) = user_id {
        req = req.data::<i64>(user_id);
    }

    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint(GRAPHQL_ENDPOINT).finish())
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

#[derive(Clone, Copy, Default)]
struct MakeRequestUuidV7;

impl MakeRequestId for MakeRequestUuidV7 {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let ret = Uuid::now_v7().to_string().parse();
        match ret {
            Ok(request_id) => Some(RequestId::new(request_id)),
            Err(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    pub async fn new(app_config: AppConfig) -> Self {
        let jwt_handler = JwtHandler::new(app_config.clone());

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

        let message_queue = MessageQueue::new(app_config.kafka.brokers.clone());

        let interactive_client = InteractiveServiceClient::connect("http://127.0.0.1:50001")
            .await
            .expect("Failed to connect to interactive service");
        let interactive_srv = InteractiveSrv::new(interactive_client);

        let comment_client = CommentServiceClient::connect("http://127.0.0.1:50002")
            .await
            .expect("Failed to connect to comment service");
        let comment_srv = CommentSrv::new(comment_client);

        let note_client = NoteServiceClient::connect("http://127.0.0.1:50003")
            .await
            .expect("Failed to connect to note service");
        let note_srv = NoteSrv::new(note_client);

        Self {
            inner: Arc::new(AppStateInner {
                app_config,
                message_queue,
                jwt_handler,
                user_srv: UserSrv::new(db.clone(), db_read.clone(), rdb),
                note_srv,
                interactive_srv,
                comment_srv,
            }),
        }
    }
}

impl Deref for AppState {
    type Target = Arc<AppStateInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct AppStateInner {
    pub(crate) message_queue: Arc<MessageQueue>,
    pub(crate) jwt_handler: JwtHandler,
    pub(crate) app_config: AppConfig,
    pub(crate) user_srv: UserSrv,
    pub(crate) note_srv: NoteSrv,
    pub(crate) interactive_srv: InteractiveSrv,
    pub(crate) comment_srv: CommentSrv,
}
