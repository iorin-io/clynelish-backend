use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChildCategory {
    pub child_category_id: Option<i32>,
    pub parent_category_id: i32,
    pub child_category_name: String,
}
