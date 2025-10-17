// post request

use axum::{Json, Router, routing::post};

use serde::Deserialize;

/// Accepts a JSON body and responds with confirmation.
#[derive(Deserialize)]
struct Message {
  content: String,
}

async fn post_message(Json(payload): Json<Message>) -> String {
  format!("Received content: {}!", payload.content)
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/message", post(post_message));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
