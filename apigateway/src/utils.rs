use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
      std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "axum_api_gateway=debug,tower_http=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init(); // 初期化
}
