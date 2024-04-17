use axum::{routing::get, Router};
use handlers::{hello, reqwest_example::get_ipv4, response_404};

mod handlers;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/get-ipv4", get(get_ipv4))
        .fallback(response_404)
}
