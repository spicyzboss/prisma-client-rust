use std::sync::Arc;

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::{DateTime, FixedOffset};
use indexmap::IndexMap;
use serde::{Serialize, Serializer};
use uuid::Uuid;

/// A Rust-friendly version of Prisma's own PrismaValue.
///
/// Prisma's PrismaValue has serialization overrides that make it suitable for JSON serialization,
/// but they lose some type information (eg. Bytes are encoded as base64), and can be less efficient
/// (eg. float values are encoded as strings).
///
/// This implementation only has an override for `PrismaValue::Null`, which is serialized as `None`
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum PrismaValue {
    String(String),
    Boolean(bool),
    Enum(String),
    Int(i32),
    Uuid(Uuid),
    List(Vec<PrismaValue>),
    Json(serde_json::Value),
    Xml(String),
    Object(Vec<(String, PrismaValue)>),
    #[serde(serialize_with = "serialize_null")]
    Null,
    DateTime(DateTime<FixedOffset>),
    Float(f64),
    BigInt(i64),
    Bytes(Vec<u8>),
}

/// A Rust-friendly version of Prisma's own Item.
/// Exists solely for nicer conversion of query results to our PrismaValue.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum Item {
    Map(IndexMap<String, Item>),
    List(Vec<Item>),
    Value(PrismaValue),
    Json(serde_json::Value),
}

impl From<query_core::Item> for Item {
    fn from(item: query_core::Item) -> Self {
        match item {
            query_core::Item::Map(map) => {
                Item::Map(map.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            query_core::Item::List(list) => {
                Item::List(list.into_iter().map(|v| v.into()).collect())
            }
            query_core::Item::Value(scalar) => Item::Value(scalar.into()),
            query_core::Item::Json(json) => Item::Json(json),
            query_core::Item::Ref(arc) => Arc::try_unwrap(arc)
                .unwrap_or_else(|arc| (*arc).to_owned())
                .into(),
        }
    }
}

fn serialize_null<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Option::<()>::None.serialize(serializer)
}

impl From<prisma_models::PrismaValue> for PrismaValue {
    fn from(value: prisma_models::PrismaValue) -> Self {
        match value {
            prisma_models::PrismaValue::String(value) => PrismaValue::String(value),
            prisma_models::PrismaValue::Boolean(value) => PrismaValue::Boolean(value),
            prisma_models::PrismaValue::Enum(value) => PrismaValue::Enum(value),
            prisma_models::PrismaValue::Int(value) => PrismaValue::Int(value as i32),
            prisma_models::PrismaValue::Uuid(value) => PrismaValue::Uuid(value.into()),
            prisma_models::PrismaValue::List(value) => {
                PrismaValue::List(value.into_iter().map(Into::into).collect())
            }
            prisma_models::PrismaValue::Json(value) => {
                PrismaValue::Json(serde_json::from_str(&value).unwrap())
            }
            prisma_models::PrismaValue::Xml(value) => PrismaValue::Xml(value),
            prisma_models::PrismaValue::Object(value) => {
                PrismaValue::Object(value.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            prisma_models::PrismaValue::Null => PrismaValue::Null,
            prisma_models::PrismaValue::DateTime(value) => PrismaValue::DateTime(value),
            prisma_models::PrismaValue::Float(value) => PrismaValue::Float(value.to_f64().unwrap()),
            prisma_models::PrismaValue::BigInt(value) => PrismaValue::BigInt(value),
            prisma_models::PrismaValue::Bytes(value) => PrismaValue::Bytes(value),
        }
    }
}

impl Into<prisma_models::PrismaValue> for PrismaValue {
    fn into(self) -> prisma_models::PrismaValue {
        match self {
            PrismaValue::String(value) => prisma_models::PrismaValue::String(value),
            PrismaValue::Boolean(value) => prisma_models::PrismaValue::Boolean(value),
            PrismaValue::Enum(value) => prisma_models::PrismaValue::Enum(value),
            PrismaValue::Int(value) => prisma_models::PrismaValue::Int(value as i64),
            PrismaValue::Uuid(value) => prisma_models::PrismaValue::Uuid(value),
            PrismaValue::List(value) => {
                prisma_models::PrismaValue::List(value.into_iter().map(Into::into).collect())
            }
            PrismaValue::Json(value) => {
                prisma_models::PrismaValue::Json(serde_json::to_string(&value).unwrap())
            }
            PrismaValue::Xml(value) => prisma_models::PrismaValue::Xml(value),
            PrismaValue::Object(value) => prisma_models::PrismaValue::Object(
                value.into_iter().map(|(k, v)| (k, v.into())).collect(),
            ),
            PrismaValue::Null => prisma_models::PrismaValue::Null,
            PrismaValue::DateTime(value) => prisma_models::PrismaValue::DateTime(value),
            PrismaValue::Float(value) => {
                prisma_models::PrismaValue::Float(BigDecimal::from_f64(value).unwrap())
            }
            PrismaValue::BigInt(value) => prisma_models::PrismaValue::BigInt(value),
            PrismaValue::Bytes(value) => prisma_models::PrismaValue::Bytes(value),
        }
    }
}
