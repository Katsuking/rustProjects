use axum::{
  Router,
  body::Bytes,
  extract::{Path, State},
  http::{HeaderMap, Method, StatusCode, Uri},
  response::{IntoResponse, Response},
  routing::{self, any, get},
};
use dotenvy::dotenv;
use reqwest::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod crates;
mod rustaceans;
mod schema;

// アプリケーションの状態として、HTTPクライアントを共有するための構造体
#[derive(Clone)]
struct AppState {
  http_client: Client,
}

/// APIゲートウェイのヘルスチェック用ハンドラ
async fn health_check() -> (StatusCode, &'static str) {
  (StatusCode::OK, "OK")
}

/// FastAPIサービスへのリクエストを転送するプロキシハンドラ
async fn proxy_handler(
  State(state): State<AppState>,
  Path(path): Path<String>,
  method: Method,
  headers: HeaderMap,
  uri: Uri,
  body: Bytes,
  fastapi_base_url: &str,
) -> Result<Response, StatusCode> {
  // 転送先のURLを構築
  // 元のリクエストのクエリパラメータも維持する
  let path_and_query = uri
    .path_and_query()
    .map(|pq| pq.as_str())
    .unwrap_or_else(|| uri.path());

  let target_url = format!("{}{}", fastapi_base_url, path_and_query);

  tracing::debug!("Forwarding request to: {} {}", method, target_url);

  // reqwestを使って、受け取ったリクエストをそのまま転送
  let response = state
    .http_client
    .request(method, &target_url)
    .headers(headers)
    .body(body)
    .send()
    .await
    .map_err(|e| {
      tracing::error!("Failed to forward request: {}", e);
      StatusCode::BAD_GATEWAY // 転送先サービスに接続できない場合は502エラー
    })?;

  // FastAPIからのレスポンスをクライアントに返す
  // ステータスコード、ヘッダー、ボディをコピーする
  let mut response_builder = Response::builder().status(response.status());
  if let Some(resp_headers) = response_builder.headers_mut() {
    *resp_headers = response.headers().clone();
  }
  let body = response
    .bytes()
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok(response_builder.body(body.into()).unwrap())
}

#[tokio::main]
async fn main() {
  println!("Hello, world! ");
  dotenv().expect(".env file not found");

  let shared_state = AppState {
    http_client: Client::new(),
  };

  // 環境変数などから読み込むのが望ましい
  let fastapi_service_url = "http://localhost:8000";

  // ログを便利に
  tracing_subscriber::fmt()
    .without_time()
    .with_target(true)
    .with_env_filter(EnvFilter::from_default_env())
    .init();

  // アプリケーションのルーティング設定
  let app = Router::new()
    // 1. APIゲートウェイ独自のエンドポイント
    .route("/health", get(health_check))
    // 2. FastAPIへのプロキシエンドポイント
    // `/*path` は、そのパス以下のすべてのリクエストにマッチする
    .route(
      "/api/v1/*path",
      any(move |State(state), path, method, headers, uri, body| {
        proxy_handler(state, path, method, headers, uri, body, fastapi_service_url)
      }),
    )
    .with_state(shared_state);

  let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
  info!("Listening on {:?}", listener.local_addr());
  axum::serve(listener, routes_all).await.unwrap();
}
