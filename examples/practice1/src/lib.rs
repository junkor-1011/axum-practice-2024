use axum::{routing::get, Router};
use handlers::{hello, reqwest_example::get_ipv4};

mod handlers;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/get-ipv4", get(get_ipv4))
}
