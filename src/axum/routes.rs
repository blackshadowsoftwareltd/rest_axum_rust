use axum::{routing::get, Router};

use super::functions::root::root;

pub async fn routes() -> Router {
    Router::new().route("/", get(root))
}
