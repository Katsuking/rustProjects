// log RUST_LOG=debug cargo run
mod utils;

use crate::utils::init_tracing;
use axum::{Router, extract::State, http, routing};
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing::info_span;

#[derive(Clone)] // Rustコンパイラが自動的に .clone() メソッドを実装して
struct AppState {
  counter: Arc<Mutex<u32>>,
}

async fn increment(State(state): State<AppState>) -> String {
  // acquire a lock on the counter
  let mut counter = state.counter.lock().unwrap();
  *counter += 1;
  format!("Counter: {}", *counter)
}

#[tokio::main]
async fn main() {
  init_tracing();

  let state = AppState {
    // 初期値を0とする
    counter: Arc::new(Mutex::new(0)),
  };
  let app = Router::new()
    .route("/increment", routing::get(increment))
    .with_state(state)
    .layer(TraceLayer::new_for_http()
    .make_span_with(|request: &http::Request<_>| {
      info_span!("http_request", method = %request.method(), uri = %request.uri(), version = ?request.version())
    })
  );
  // Define the address and port the server will bind to.

  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
