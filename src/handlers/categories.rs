use axum::{
    extract::{Json, Extension, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::{query_as, query};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::db::AppState;
use crate::models::{parent_category::ParentCategory, child_category::ChildCategory};

pub async fn create_parent_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(category): Json<ParentCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        ParentCategory,
        "INSERT INTO ParentCategories (account_id, parent_category_name, color, category_type) VALUES (?, ?, ?, ?) RETURNING parent_category_id, account_id, parent_category_name, color, category_type",
        category.account_id,
        category.parent_category_name,
        category.color,
        category.category_type as i32
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(new_category) => (StatusCode::CREATED, Json(new_category)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_child_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(category): Json<ChildCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        ChildCategory,
        "INSERT INTO ChildCategories (parent_category_id, child_category_name) VALUES (?, ?) RETURNING child_category_id, parent_category_id, child_category_name",
        category.parent_category_id,
        category.child_category_name
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(new_category) => (StatusCode::CREATED, Json(new_category)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_categories(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(account_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    let parent_categories: Vec<ParentCategory> = match query_as!(
        ParentCategory,
        "SELECT parent_category_id, account_id, parent_category_name, color, category_type FROM ParentCategories WHERE account_id = ?",
        account_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(categories) => categories,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let child_categories: Vec<ChildCategory> = match query_as!(
        ChildCategory,
        "SELECT child_category_id, parent_category_id, child_category_name FROM ChildCategories WHERE parent_category_id IN (SELECT parent_category_id FROM ParentCategories WHERE account_id = ?)",
        account_id
    )
    .fetch_all(&db_pool)
    .await
    {
        Ok(categories) => categories,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let response = (parent_categories, child_categories);
    (StatusCode::OK, Json(response)).into_response()
}

pub async fn update_parent_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(parent_category_id): Path<i32>,
    Json(category): Json<ParentCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        ParentCategory,
        "UPDATE ParentCategories SET parent_category_name = ?, color = ?, category_type = ? WHERE parent_category_id = ? RETURNING parent_category_id, account_id, parent_category_name, color, category_type",
        category.parent_category_name,
        category.color,
        category.category_type as i32,
        parent_category_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(updated_category) => (StatusCode::OK, Json(updated_category)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_child_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(child_category_id): Path<i32>,
    Json(category): Json<ChildCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query_as!(
        ChildCategory,
        "UPDATE ChildCategories SET child_category_name = ? WHERE child_category_id = ? RETURNING child_category_id, parent_category_id, child_category_name",
        category.child_category_name,
        child_category_id
    )
    .fetch_one(&db_pool)
    .await
    {
        Ok(updated_category) => (StatusCode::OK, Json(updated_category)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_parent_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(parent_category_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "DELETE FROM ParentCategories WHERE parent_category_id = ?",
        parent_category_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_child_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(child_category_id): Path<i32>,
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "DELETE FROM ChildCategories WHERE child_category_id = ?",
        child_category_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
