use axum::{Router, routing::get, routing::post, routing::put};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::AppState;

use crate::handlers::{
    users::{create_user, get_users, get_user, update_user, delete_user},
    accounts::{create_account, get_account, update_account, delete_account},
    categories::{create_parent_category, create_child_category, get_categories, update_parent_category, update_child_category, delete_parent_category, delete_child_category},
    transactions::{create_transaction, get_transaction, update_transaction, delete_transaction},
};

pub fn create_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }).post(|| async { "Hello, world!" }))
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/accounts", post(create_account))
        .route("/accounts/:id", get(get_account).put(update_account).delete(delete_account))
        .route("/categories/parent", post(create_parent_category))
        .route("/categories/child", post(create_child_category))
        .route("/categories/:id", get(get_categories))
        .route("/categories/parent/:id", put(update_parent_category).delete(delete_parent_category))
        .route("/categories/child/:id", put(update_child_category).delete(delete_child_category))
        .route("/transactions", post(create_transaction))
        .route("/transactions/:id", get(get_transaction).put(update_transaction).delete(delete_transaction))
        .layer(axum::Extension(state))
}
