use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

const LOG_LEVEL_ENV: &str = "RUST_LOG";

#[tokio::main]
async fn main() {
    let log_level = env::var(LOG_LEVEL_ENV).unwrap_or("info".to_string());
    env::set_var(LOG_LEVEL_ENV, log_level);
    tracing_subscriber::fmt::init();
    // tracing_subscriber::fmt().json().init();

    let app = create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/greet", get(greet))
        .route("/greet", post(greet_post))
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn greet() -> impl IntoResponse {
    tracing::debug!("greet");

    let body = GreetGetResponse {
        message: "Hello, World!!!".to_string(),
        date: Utc::now().to_string(),
    };

    (StatusCode::OK, Json(body))
}

async fn greet_post(Json(payload): Json<GreetPostRequest>) -> impl IntoResponse {
    let GreetPostRequest { name } = payload;
    let body = GreetPostResponse {
        message: format!("Hello, {name}"),
        date: Utc::now().to_string(),
    };

    (StatusCode::OK, Json(body))
}

#[derive(Serialize, Debug)]
struct GreetGetResponse {
    message: String,
    date: String,
}

#[derive(Deserialize)]
struct GreetPostRequest {
    name: String,
}

#[derive(Serialize, Debug)]
struct GreetPostResponse {
    message: String,
    date: String,
}
