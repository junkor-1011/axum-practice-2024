use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use handlers::{hello, post_json_example, reqwest_example::get_ipv4, response_404};

mod handlers;
pub mod utils;

pub fn create_app() -> Router {
    let reqwest_client = reqwest::Client::new();

    let state = AppState { reqwest_client };

    Router::new()
        .route("/", get(hello))
        .route("/get-ipv4", get(get_ipv4))
        .route("/item/:item/id/:id", post(post_json_example::handler))
        .fallback(response_404)
        .with_state(state)
}

#[derive(Clone, FromRef)]
pub struct AppState {
    reqwest_client: reqwest::Client,
}
