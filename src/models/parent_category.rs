use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ParentCategory {
    pub parent_category_id: Option<i32>,
    pub account_id: i32,
    pub parent_category_name: String,
    pub color: String,
    pub category_type: String,
}
