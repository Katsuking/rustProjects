// extract query params

use axum::{Router, extract::Query, routing};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
  page: Option<u32>,
  limit: Option<u32>,
}

async fn list_items(Query(params): Query<Pagination>) -> String {
  format!(
    "Listing items on page {:?} with limit {:?}!",
    params.page, params.limit
  )
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/items", routing::get(list_items));
  // Define the address and port the server will bind to.
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();
  // Start the server. `axum::Server` runs indefinitely until stopped.
  axum::serve(listener, app).await.unwrap();
}
