use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum CategoryType {
    Income = 1,
    Expense = 2,
}

#[derive(Deserialize, Serialize)]
pub struct ParentCategory {
    pub parent_category_id: Option<i32>,
    pub account_id: i32,
    pub parent_category_name: String,
    pub color: String,
    pub category_type: CategoryType,
}
