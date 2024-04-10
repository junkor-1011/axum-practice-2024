use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
use handlers::{greet, greet_post, root};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/greet", get(greet))
        .route("/greet", post(greet_post))
}
