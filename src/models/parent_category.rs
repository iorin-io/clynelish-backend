use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::types::Type;
use sqlx::{Encode, Decode, Postgres, postgres::PgTypeInfo};
use std::error::Error;
use std::fmt;

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CategoryType {
    Income = 1,
    Expense = 2,
}

impl Type<Postgres> for CategoryType {
    fn type_info() -> PgTypeInfo {
        <i32 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for CategoryType {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        <i32 as Encode<Postgres>>::encode_by_ref(&(*self as i32), buf)
    }
}

impl<'r> Decode<'r, Postgres> for CategoryType {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let v = <i32 as Decode<Postgres>>::decode(value)?;
        match v {
            1 => Ok(CategoryType::Income),
            2 => Ok(CategoryType::Expense),
            _ => Err(Box::new(InvalidCategoryTypeError(v))),
        }
    }
}

#[derive(Debug)]
struct InvalidCategoryTypeError(i32);

impl fmt::Display for InvalidCategoryTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid value {} for CategoryType", self.0)
    }
}

impl Error for InvalidCategoryTypeError {}

// From<i32> トレイトを実装
impl From<i32> for CategoryType {
    fn from(value: i32) -> Self {
        match value {
            1 => CategoryType::Income,
            2 => CategoryType::Expense,
            _ => panic!("Invalid value for CategoryType"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ParentCategory {
    pub parent_category_id: Option<i32>,
    pub account_id: i32,
    pub parent_category_name: String,
    pub color: String,
    pub category_type: CategoryType,
}
