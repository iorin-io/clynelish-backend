use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::account::Account;

pub async fn create_account(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_account): Json<Account>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO Accounts (user_id, account_name, initial_balance) VALUES (?, ?, ?)",
        new_account.user_id,
        new_account.account_name,
        new_account.initial_balance
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            let account_id = result.last_insert_id();
            match query_as!(
                Account,
                "SELECT account_id, user_id, account_name, initial_balance, created_at FROM Accounts WHERE account_id = ?",
                account_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch account after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create account: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_account(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(account_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        Account,
        "SELECT account_id, user_id, account_name, initial_balance, created_at FROM Accounts WHERE account_id = ?",
        account_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(account) => (StatusCode::OK, Json(account)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_account(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(account_id): Path<i32>,
    Json(account): Json<Account>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE Accounts SET account_name = ?, initial_balance = ? WHERE account_id = ?",
        account.account_name,
        account.initial_balance,
        account_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                Account,
                "SELECT account_id, user_id, account_name, initial_balance, created_at FROM Accounts WHERE account_id = ?",
                account_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(updated_account) => (StatusCode::OK, Json(updated_account)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch account after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update account: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_account(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(account_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "DELETE FROM Accounts WHERE account_id = ?",
        account_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
