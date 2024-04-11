use practice1::create_app;
use std::{env, net::SocketAddr};

const LOG_LEVEL_ENV: &str = "RUST_LOG";

#[tokio::main]
async fn main() {
    let log_level = env::var(LOG_LEVEL_ENV).unwrap_or("info".to_string());
    env::set_var(LOG_LEVEL_ENV, log_level);

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = create_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
