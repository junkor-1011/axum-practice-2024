use axum::{routing::get, Router};
use std::env;
use std::net::SocketAddr;

const LOG_LEVEL_ENV: &str = "RUST_LOG";

#[tokio::main]
async fn main() {
    let log_level = env::var(LOG_LEVEL_ENV).unwrap_or("info".to_string());
    env::set_var(LOG_LEVEL_ENV, log_level);
    tracing_subscriber::fmt::init();
    // tracing_subscriber::fmt().json().init();

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
