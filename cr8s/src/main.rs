// path param パスパラメーターを受け取るには?

use axum::{Router, extract::Path, routing::get};

/// Handler that greets the user by name.
/// The `Path<String>` extractor automatically parses the `:name` parameter from the URL.
async fn greet(Path(name): Path<String>) -> String {
  format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/greet/{name}", get(greet));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
