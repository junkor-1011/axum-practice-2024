use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub mod post_json_example;
pub mod reqwest_example;

#[tracing::instrument]
pub async fn hello() -> Html<String> {
    tracing::info!("hello invoked.");

    let dt = chrono::Utc::now();

    Html(format!(
        "
<html>
  <head>
    <title>example</title>
  </head>
  <body>
    <h1>Hello, World!</h1>
    <p>now: {}</p>
  </body>
</html>",
        dt
    ))
}

#[tracing::instrument]
pub async fn response_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        serde_json::json!({
          "status": StatusCode::NOT_FOUND.as_u16(),
          "message": "not found",
        })
        .to_string(),
    )
}
