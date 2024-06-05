use axum::{Router, routing::get, routing::post, routing::put, routing::delete};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::AppState;

use crate::handlers::{
    users::{create_user, get_users, get_user, update_user, delete_user},
    accounts::{create_account, get_account, update_account, delete_account},
    categories::{create_parent_category, create_child_category, get_categories, update_parent_category, update_child_category, delete_parent_category, delete_child_category},
    transactions::{create_transaction, get_transaction, update_transaction, delete_transaction},
    budgets::{create_budget, get_budget, update_budget, delete_budget},
};

pub fn create_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .route("/accounts", post(create_account))
        .route("/accounts/:id", get(get_account))
        .route("/accounts/:id", put(update_account))
        .route("/accounts/:id", delete(delete_account))
        .route("/categories/parent", post(create_parent_category))
        .route("/categories/child", post(create_child_category))
        .route("/categories/:id", get(get_categories))
        .route("/categories/parent/:id", put(update_parent_category))
        .route("/categories/child/:id", put(update_child_category))
        .route("/categories/parent/:id", delete(delete_parent_category))
        .route("/categories/child/:id", delete(delete_child_category))
        .route("/transactions", post(create_transaction))
        .route("/transactions/:id", get(get_transaction))
        .route("/transactions/:id", put(update_transaction))
        .route("/transactions/:id", delete(delete_transaction))
        .route("/budgets", post(create_budget))
        .route("/budgets/:id", get(get_budget))
        .route("/budgets/:id", put(update_budget))
        .route("/budgets/:id", delete(delete_budget))
        .layer(axum::Extension(state))
}
