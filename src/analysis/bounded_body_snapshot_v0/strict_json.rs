//! Strict JSON syntax boundary with duplicate-key and trailing-input rejection.

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum StrictJsonValue {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Array(Vec<StrictJsonValue>),
    Object(Vec<(String, StrictJsonValue)>),
}

impl StrictJsonValue {
    pub(super) fn kind_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "bool",
            Self::I64(_) | Self::U64(_) | Self::F64(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

    pub(super) fn object_field(&self, name: &str) -> Option<&StrictJsonValue> {
        let Self::Object(fields) = self else {
            return None;
        };
        fields
            .iter()
            .find_map(|(key, value)| (key == name).then_some(value))
    }

    pub(super) fn object_fields(&self) -> Option<&[(String, StrictJsonValue)]> {
        match self {
            Self::Object(fields) => Some(fields),
            _ => None,
        }
    }

    pub(super) fn array_items(&self) -> Option<&[StrictJsonValue]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }

    pub(super) fn string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub(super) fn exact_i64(&self) -> Option<i64> {
        match self {
            Self::I64(value) => Some(*value),
            Self::U64(value) => i64::try_from(*value).ok(),
            Self::String(value) => parse_canonical_i64(value),
            _ => None,
        }
    }

    pub(super) fn json_integer_i64(&self) -> Option<i64> {
        match self {
            Self::I64(value) => Some(*value),
            Self::U64(value) => i64::try_from(*value).ok(),
            _ => None,
        }
    }
}

fn parse_canonical_i64(value: &str) -> Option<i64> {
    if value.is_empty() || value.starts_with('+') {
        return None;
    }
    let digits = value.strip_prefix('-').unwrap_or(value);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if digits.len() > 1 && digits.starts_with('0') {
        return None;
    }
    value.parse().ok()
}

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}

struct StrictJsonVisitor;

impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value without duplicate object keys")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::Null)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::I64(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::U64(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::F64(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictJsonValue::String(value))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(item) = sequence.next_element()? {
            items.push(item);
        }
        Ok(StrictJsonValue::Array(items))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fields = Vec::new();
        let mut seen = HashSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!("duplicate key: {key}")));
            }
            fields.push((key, map.next_value()?));
        }
        Ok(StrictJsonValue::Object(fields))
    }
}

pub(super) fn parse_strict_json(input: &str) -> Result<StrictJsonValue, String> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let value =
        StrictJsonValue::deserialize(&mut deserializer).map_err(|error| error.to_string())?;
    deserializer.end().map_err(|error| error.to_string())?;
    Ok(value)
}
