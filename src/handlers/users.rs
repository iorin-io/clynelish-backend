use axum::{
    extract::{Json, Extension},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use chrono::NaiveDateTime;  // 追加

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,  // 変更
}

pub async fn create_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        "INSERT INTO Users (username, email, password) VALUES ($1, $2, $3) RETURNING user_id, username, email, password, created_at",
        user.username,
        user.email,
        user.password
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(new_user) => (StatusCode::CREATED, Json(new_user)).into_response(),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        },
    }
}
