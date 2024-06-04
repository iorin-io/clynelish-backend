use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<String>,
}