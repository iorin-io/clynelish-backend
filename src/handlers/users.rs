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
    Json(new_user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO Users (username, user_email, firebase_uid) VALUES (?, ?, ?)",
        new_user.username,
        new_user.user_email,
        new_user.firebase_uid
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            let user_id = result.last_insert_id();
            match query_as!(
                User,
                "SELECT user_id, username, user_email, firebase_uid, created_at FROM Users WHERE user_id = ?",
                user_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch user after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        "SELECT user_id, username, user_email, firebase_uid, created_at FROM Users WHERE user_id = ?",
        user_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_users(
    Extension(state): Extension<Arc<Mutex<AppState>>>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        User,
        "SELECT user_id, username, user_email, firebase_uid, created_at FROM Users"
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            eprintln!("Failed to fetch users: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
    Json(user): Json<User>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE Users SET username = ?, user_email = ?, firebase_uid = ? WHERE user_id = ?",
        user.username,
        user.user_email,
        user.firebase_uid,
        user_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                User,
                "SELECT user_id, username, user_email, firebase_uid, created_at FROM Users WHERE user_id = ?",
                user_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(updated_user) => (StatusCode::OK, Json(updated_user)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch user after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_user(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "DELETE FROM Users WHERE user_id = ?",
        user_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
