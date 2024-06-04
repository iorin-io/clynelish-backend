use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use sqlx::types::BigDecimal;
use crate::serializers::bigdecimal_serde;

#[derive(Deserialize, Serialize)]
pub struct Budget {
    pub budget_id: Option<i32>,
    pub user_id: i32,
    pub child_category_id: i32,
    #[serde(with = "bigdecimal_serde")]
    pub amount: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}
