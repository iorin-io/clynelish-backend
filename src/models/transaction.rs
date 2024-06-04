use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_id: Option<i32>,
    pub account_id: i32,
    pub child_category_id: i32,
    pub transaction_amount: f64,
    pub transaction_type: String,
    pub transaction_date: NaiveDate,
    pub transaction_description: Option<String>,
}
