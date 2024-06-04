use axum::{Router, routing::post};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::AppState;

use crate::handlers::users::create_user;

pub fn create_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/users", post(create_user))

        .layer(axum::Extension(state))
}