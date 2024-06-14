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
    Json(new_category): Json<ParentCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO ParentCategories (account_id, parent_category_name, color, category_type) VALUES (?, ?, ?, ?)",
        new_category.account_id,
        new_category.parent_category_name,
        new_category.color,
        new_category.category_type as i32
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            let parent_category_id = result.last_insert_id();
            match query_as!(
                ParentCategory,
                "SELECT parent_category_id, account_id, parent_category_name, color, category_type FROM ParentCategories WHERE parent_category_id = ?",
                parent_category_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(category) => (StatusCode::CREATED, Json(category)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch parent category after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create parent category: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_child_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Json(new_category): Json<ChildCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "INSERT INTO ChildCategories (parent_category_id, child_category_name) VALUES (?, ?)",
        new_category.parent_category_id,
        new_category.child_category_name
    )
    .execute(&db_pool)
    .await
    {
        Ok(result) => {
            let child_category_id = result.last_insert_id();
            match query_as!(
                ChildCategory,
                "SELECT child_category_id, parent_category_id, child_category_name FROM ChildCategories WHERE child_category_id = ?",
                child_category_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(category) => (StatusCode::CREATED, Json(category)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch child category after creation: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create child category: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
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

    match query!(
        "UPDATE ParentCategories SET parent_category_name = ?, color = ?, category_type = ? WHERE parent_category_id = ?",
        category.parent_category_name,
        category.color,
        category.category_type as i32,
        parent_category_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                ParentCategory,
                "SELECT parent_category_id, account_id, parent_category_name, color, category_type FROM ParentCategories WHERE parent_category_id = ?",
                parent_category_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(updated_category) => (StatusCode::OK, Json(updated_category)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch parent category after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update parent category: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_child_category(
    Extension(state): Extension<Arc<Mutex<AppState>>>,
    Path(child_category_id): Path<i32>,
    Json(category): Json<ChildCategory>
) -> impl IntoResponse {
    let db_pool = state.lock().await.db_pool.clone();

    match query!(
        "UPDATE ChildCategories SET child_category_name = ? WHERE child_category_id = ?",
        category.child_category_name,
        child_category_id
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            match query_as!(
                ChildCategory,
                "SELECT child_category_id, parent_category_id, child_category_name FROM ChildCategories WHERE child_category_id = ?",
                child_category_id
            )
            .fetch_one(&db_pool)
            .await {
                Ok(updated_category) => (StatusCode::OK, Json(updated_category)).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch child category after update: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to update child category: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
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
