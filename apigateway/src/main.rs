// Customizing Headers with Response

use axum::{
  Router,
  http::{HeaderValue, StatusCode},
  response::Response,
  routing,
};
use hyper::header;

async fn custom_header() -> Response {
  let body = "custom body example";
  Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "text/plain")
    .header("X-App-Version", HeaderValue::from_static("1.0"))
    .body(body.into())
    .unwrap()
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/custom_header", routing::get(custom_header));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
