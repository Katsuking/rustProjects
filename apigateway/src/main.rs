// Writing Custom Middleware

use std::sync::{Arc, Mutex};

use axum::{Router, extract::State, routing};

#[derive(Clone)]
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
  let state = AppState {
    // 初期値を0とする
    counter: Arc::new(Mutex::new(0)),
  };
  let app = Router::new()
    .route("/increment", routing::get(increment))
    .with_state(state);
  // Define the address and port the server will bind to.

  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
