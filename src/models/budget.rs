use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct Budget {
    pub budget_id: Option<i32>,
    pub user_id: i32,
    pub child_category_id: i32,
    pub amount: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}
