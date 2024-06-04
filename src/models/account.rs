use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use crate::serializers::bigdecimal_serde;

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub account_id: Option<i32>,
    pub user_id: i32,
    pub account_name: String,
    #[serde(with = "bigdecimal_serde")]
    pub initial_balance: BigDecimal,
    pub created_at: Option<NaiveDateTime>,
}
