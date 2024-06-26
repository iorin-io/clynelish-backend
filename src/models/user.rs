use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: String,
    pub user_email: String,
    pub user_password: String,
    pub created_at: Option<NaiveDateTime>,
}