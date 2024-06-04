use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use sqlx::types::BigDecimal;
use crate::serializers::bigdecimal_serde;

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_id: Option<i32>,
    pub account_id: i32,
    pub child_category_id: i32,
    #[serde(with = "bigdecimal_serde")]
    pub transaction_amount: BigDecimal,
    pub transaction_type: String,
    pub transaction_date: NaiveDate,
    pub transaction_description: Option<String>,
}
