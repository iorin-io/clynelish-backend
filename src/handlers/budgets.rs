use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::budget::Budget;

pub async fn create_budget(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(budget): Json<Budget>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Budget,
        "INSERT INTO Budgets (user_id, child_category_id, amount, start_date, end_date) VALUES ($1, $2, $3, $4, $5) RETURNING budget_id, user_id, child_category_id, amount, start_date, end_date",
        budget.user_id,
        budget.child_category_id,
        budget.amount,
        budget.start_date,
        budget.end_date
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(new_budget) => (StatusCode::CREATED, Json(new_budget)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_budget(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(budget_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Budget,
        "SELECT budget_id, user_id, child_category_id, amount, start_date, end_date FROM Budgets WHERE budget_id = $1",
        budget_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(budget) => (StatusCode::OK, Json(budget)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_budget(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(budget_id): Path<i32>,
    Json(budget): Json<Budget>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Budget,
        "UPDATE Budgets SET amount = $1, start_date = $2, end_date = $3 WHERE budget_id = $4 RETURNING budget_id, user_id, child_category_id, amount, start_date, end_date",
        budget.amount,
        budget.start_date,
        budget.end_date,
        budget_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(updated_budget) => (StatusCode::OK, Json(updated_budget)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_budget(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(budget_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "DELETE FROM Budgets WHERE budget_id = $1",
        budget_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
