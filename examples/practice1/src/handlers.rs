use axum::response::Html;

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
