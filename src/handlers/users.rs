use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::user::User;

pub async fn create_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        "INSERT INTO Users (username, user_email, user_password) VALUES ($1, $2, $3) RETURNING user_id, username, user_email, user_password, created_at",
        user.username,
        user.user_email,
        user.user_password
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(new_user) => (StatusCode::CREATED, Json(new_user)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        "SELECT user_id, username, user_email, user_password, created_at FROM Users WHERE user_id = $1",
        user_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
    Json(user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        "UPDATE Users SET username = $1, user_email = $2, user_password = $3 WHERE user_id = $4 RETURNING user_id, username, user_email, user_password, created_at",
        user.username,
        user.user_email,
        user.user_password,
        user_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(updated_user) => (StatusCode::OK, Json(updated_user)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "DELETE FROM Users WHERE user_id = $1",
        user_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
