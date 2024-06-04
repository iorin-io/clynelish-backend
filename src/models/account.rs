use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub account_id: Option<i32>,
    pub user_id: i32,
    pub account_name: String,
    pub initial_balance: f64,
    pub created_at: Option<NaiveDateTime>,
}
