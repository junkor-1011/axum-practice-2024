use axum::{routing::get, Router};
use handlers::hello;

mod handlers;

pub fn create_app() -> Router {
    Router::new().
        route("/", get(hello))
}
