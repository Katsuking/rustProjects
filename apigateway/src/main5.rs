// returning JSON response

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
  message: String,
  success: bool,
}

async fn create_user() -> impl IntoResponse {
  let res = ApiResponse {
    message: "User created successfully".to_string(),
    success: true,
  };
  (StatusCode::CREATED, Json(res))
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/user", routing::get(create_user));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
