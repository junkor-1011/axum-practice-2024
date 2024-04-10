use axum::{response::IntoResponse, Json};
use chrono::Utc;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub async fn root() -> &'static str {
    "Hello, world!"
}

pub async fn greet() -> impl IntoResponse {
    tracing::debug!("greet");

    let body = GreetGetResponse {
        message: "Hello, World!!!".to_string(),
        date: Utc::now().to_string(),
    };

    (StatusCode::OK, Json(body))
}

pub async fn greet_post(Json(payload): Json<GreetPostRequest>) -> impl IntoResponse {
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
pub struct GreetPostRequest {
    name: String,
}

#[derive(Serialize, Debug)]
struct GreetPostResponse {
    message: String,
    date: String,
}
