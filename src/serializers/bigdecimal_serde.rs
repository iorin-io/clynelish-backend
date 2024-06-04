use serde::{self, Deserialize, Deserializer, Serializer};
use sqlx::types::BigDecimal;
use std::str::FromStr;

pub fn serialize<S>(value: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BigDecimal::from_str(&s).map_err(serde::de::Error::custom)
}