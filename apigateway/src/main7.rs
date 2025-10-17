// Writing Custom Middleware

use axum::{
  Router,
  http::{HeaderValue, Request, StatusCode},
  middleware::{self, Next},
  response::Response,
  routing,
};
use hyper::header;

/// Middleware that ensures every request has an Authorization header.
async fn require_auth(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
  if req.headers().get("Authorization").is_some() {
    println!("req: {:?}", req);
    Ok(next.run(req).await)
  } else {
    Err(StatusCode::UNAUTHORIZED)
  }
}

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
  let app = Router::new()
    .route("/custom_header", routing::post(custom_header))
    .layer(middleware::from_fn(require_auth));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
