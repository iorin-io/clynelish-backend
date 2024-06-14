use serde::{Deserialize, Serialize};
use time::Date;
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
    pub transaction_date: Date,
    pub transaction_description: Option<String>,
}
