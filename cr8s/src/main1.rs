use axum::{Router, routing::get};
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
  "Hello, World!"
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/", get(hello_world));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
