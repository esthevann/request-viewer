// Code generated by Prisma Client Rust. DO NOT EDIT

#![allow(warnings, unused)]
use prisma_client_rust::{
    bigdecimal::{self, FromPrimitive},
    chrono,
    datamodel::parse_configuration,
    operator::Operator,
    prisma_models::{InternalDataModelBuilder, PrismaValue},
    queries::{QueryContext, QueryInfo, Result as QueryResult},
    query_core::{
        executor, schema_builder, BuildMode, CoreError, InterpreterError, QueryExecutor,
        QueryGraphBuilderError, QuerySchema, QueryValue, Selection,
    },
    serde_json, transform_equals, BatchResult, Direction, ManyArgs, SerializedWhere,
    SerializedWhereValue, UniqueArgs,
};
pub use prisma_client_rust::{queries::Error as QueryError, NewClientError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
static DATAMODEL_STR : & 'static str = "datasource db {\r\n    provider = \"postgresql\"\r\n    url = \"postgresql://postgres:yoyHUDvrrZPDN6X2@db.rwryoviujjinfuqxoszm.supabase.co:5432/postgres\"\r\n}\r\n\r\ngenerator client {\r\n    provider = \"cargo prisma\"\r\n    output = \"./mod.rs\"\r\n}\r\n\r\nmodel Request {\r\n    id   String @id @default(cuid())\r\n    address String?\r\n    name String\r\n    createdAt DateTime @default(now())\r\n    method Method @default(GET)\r\n}\r\n\r\nenum Method {\r\n    GET\r\n    POST\r\n    PUT\r\n    PATCH\r\n    DELETE\r\n}" ;
static DATABASE_STR: &'static str = "postgresql";
pub async fn new_client() -> Result<_prisma::PrismaClient, NewClientError> {
    let config = parse_configuration(DATAMODEL_STR)?.subject;
    let source = config
        .datasources
        .first()
        .expect("Pleasy supply a datasource in your schema.prisma file");
    let url = if let Some(url) = source.load_shadow_database_url()? {
        url
    } else {
        source.load_url(|key| std::env::var(key).ok())?
    };
    let url = if url.starts_with("file:") {
        let path = url.split(":").nth(1).unwrap();
        if Path::new("./schema.prisma").exists() {
            url
        } else if Path::new("./prisma/schema.prisma").exists() {
            format!("file:./prisma/{}", path)
        } else {
            url
        }
    } else {
        url
    };
    new_client_with_url(&url).await
}
pub async fn new_client_with_url(url: &str) -> Result<_prisma::PrismaClient, NewClientError> {
    let config = parse_configuration(DATAMODEL_STR)?.subject;
    let source = config
        .datasources
        .first()
        .expect("Pleasy supply a datasource in your schema.prisma file");
    let (db_name, executor) = executor::load(&source, &[], &url).await?;
    let internal_model = InternalDataModelBuilder::new(DATAMODEL_STR).build(db_name);
    let query_schema = Arc::new(schema_builder::build(
        internal_model,
        BuildMode::Modern,
        true,
        source.capabilities(),
        vec![],
        source.referential_integrity(),
    ));
    executor.primary_connector().get_connection().await?;
    Ok(PrismaClient::_new(executor, query_schema))
}
pub mod request {
    use super::_prisma::*;
    use super::*;
    pub mod id {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: String) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn cursor(cursor: String) -> Cursor {
            Cursor::Id(cursor)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::IdInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::IdNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::IdLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::IdLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::IdGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::IdGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::IdContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::IdStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::IdEndsWith(value)
        }
        pub fn mode(value: QueryMode) -> WhereParam {
            WhereParam::IdMode(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::IdNot(value)
        }
        pub struct Set(String);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetId(value.0)
            }
        }
    }
    pub mod address {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: Option<String>) -> T {
            Set(value).into()
        }
        pub fn equals(value: Option<String>) -> WhereParam {
            WhereParam::AddressEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Address(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::AddressInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::AddressNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::AddressLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::AddressLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::AddressGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::AddressGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::AddressContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::AddressStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::AddressEndsWith(value)
        }
        pub fn mode(value: QueryMode) -> WhereParam {
            WhereParam::AddressMode(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::AddressNot(value)
        }
        pub struct Set(Option<String>);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetAddress(value.0)
            }
        }
    }
    pub mod name {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn equals(value: String) -> WhereParam {
            WhereParam::NameEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Name(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::NameInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::NameNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::NameLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::NameLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::NameGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::NameGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::NameContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::NameStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::NameEndsWith(value)
        }
        pub fn mode(value: QueryMode) -> WhereParam {
            WhereParam::NameMode(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::NameNot(value)
        }
        pub struct Set(String);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetName(value.0)
            }
        }
    }
    pub mod created_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: chrono::DateTime<chrono::FixedOffset>) -> T {
            Set(value).into()
        }
        pub fn equals(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::CreatedAt(direction)
        }
        pub fn in_vec(value: Vec<chrono::DateTime<chrono::FixedOffset>>) -> WhereParam {
            WhereParam::CreatedAtInVec(value)
        }
        pub fn not_in_vec(value: Vec<chrono::DateTime<chrono::FixedOffset>>) -> WhereParam {
            WhereParam::CreatedAtNotInVec(value)
        }
        pub fn lt(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtLt(value)
        }
        pub fn lte(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtLte(value)
        }
        pub fn gt(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtGt(value)
        }
        pub fn gte(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtGte(value)
        }
        pub fn not(value: chrono::DateTime<chrono::FixedOffset>) -> WhereParam {
            WhereParam::CreatedAtNot(value)
        }
        pub struct Set(chrono::DateTime<chrono::FixedOffset>);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetCreatedAt(value.0)
            }
        }
    }
    pub mod method {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: Method) -> T {
            Set(value).into()
        }
        pub fn equals(value: Method) -> WhereParam {
            WhereParam::MethodEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Method(direction)
        }
        pub struct Set(Method);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetMethod(value.0)
            }
        }
    }
    pub fn _outputs() -> Vec<Selection> {
        ["id", "address", "name", "createdAt", "method"]
            .into_iter()
            .map(|o| {
                let builder = Selection::builder(o);
                builder.build()
            })
            .collect()
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Data {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "address")]
        pub address: Option<String>,
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "createdAt")]
        pub created_at: chrono::DateTime<chrono::FixedOffset>,
        #[serde(rename = "method")]
        pub method: Method,
    }
    impl Data {}
    #[derive(Clone)]
    pub enum WithParam {}
    impl Into<Selection> for WithParam {
        fn into(self) -> Selection {
            match self {}
        }
    }
    #[derive(Clone)]
    pub enum SetParam {
        SetId(String),
        SetAddress(Option<String>),
        SetName(String),
        SetCreatedAt(chrono::DateTime<chrono::FixedOffset>),
        SetMethod(Method),
    }
    impl Into<(String, PrismaValue)> for SetParam {
        fn into(self) -> (String, PrismaValue) {
            match self {
                SetParam::SetId(value) => ("id".to_string(), PrismaValue::String(value)),
                SetParam::SetAddress(value) => (
                    "address".to_string(),
                    value
                        .map(|value| PrismaValue::String(value))
                        .unwrap_or(PrismaValue::Null),
                ),
                SetParam::SetName(value) => ("name".to_string(), PrismaValue::String(value)),
                SetParam::SetCreatedAt(value) => {
                    ("createdAt".to_string(), PrismaValue::DateTime(value))
                }
                SetParam::SetMethod(value) => {
                    ("method".to_string(), PrismaValue::Enum(value.to_string()))
                }
            }
        }
    }
    #[derive(Clone)]
    pub enum OrderByParam {
        Id(Direction),
        Address(Direction),
        Name(Direction),
        CreatedAt(Direction),
        Method(Direction),
    }
    impl Into<(String, PrismaValue)> for OrderByParam {
        fn into(self) -> (String, PrismaValue) {
            match self {
                Self::Id(direction) => {
                    ("id".to_string(), PrismaValue::String(direction.to_string()))
                }
                Self::Address(direction) => (
                    "address".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
                Self::Name(direction) => (
                    "name".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
                Self::CreatedAt(direction) => (
                    "createdAt".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
                Self::Method(direction) => (
                    "method".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum Cursor {
        Id(String),
    }
    impl Into<(String, PrismaValue)> for Cursor {
        fn into(self) -> (String, PrismaValue) {
            match self {
                Self::Id(cursor) => ("id".to_string(), PrismaValue::String(cursor)),
            }
        }
    }
    #[derive(Clone)]
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        IdEquals(String),
        IdInVec(Vec<String>),
        IdNotInVec(Vec<String>),
        IdLt(String),
        IdLte(String),
        IdGt(String),
        IdGte(String),
        IdContains(String),
        IdStartsWith(String),
        IdEndsWith(String),
        IdMode(QueryMode),
        IdNot(String),
        AddressEquals(Option<String>),
        AddressInVec(Vec<String>),
        AddressNotInVec(Vec<String>),
        AddressLt(String),
        AddressLte(String),
        AddressGt(String),
        AddressGte(String),
        AddressContains(String),
        AddressStartsWith(String),
        AddressEndsWith(String),
        AddressMode(QueryMode),
        AddressNot(String),
        NameEquals(String),
        NameInVec(Vec<String>),
        NameNotInVec(Vec<String>),
        NameLt(String),
        NameLte(String),
        NameGt(String),
        NameGte(String),
        NameContains(String),
        NameStartsWith(String),
        NameEndsWith(String),
        NameMode(QueryMode),
        NameNot(String),
        CreatedAtEquals(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtInVec(Vec<chrono::DateTime<chrono::FixedOffset>>),
        CreatedAtNotInVec(Vec<chrono::DateTime<chrono::FixedOffset>>),
        CreatedAtLt(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtLte(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtGt(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtGte(chrono::DateTime<chrono::FixedOffset>),
        CreatedAtNot(chrono::DateTime<chrono::FixedOffset>),
        MethodEquals(Method),
    }
    impl Into<SerializedWhere> for WhereParam {
        fn into(self) -> SerializedWhere {
            match self {
                Self::Not(value) => (
                    "NOT".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| PrismaValue::Object(transform_equals(vec![v].into_iter())))
                            .collect(),
                    ),
                ),
                Self::Or(value) => (
                    "OR".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| PrismaValue::Object(transform_equals(vec![v].into_iter())))
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| PrismaValue::Object(transform_equals(vec![v].into_iter())))
                            .collect(),
                    ),
                ),
                Self::IdEquals(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdInVec(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::IdNotInVec(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::IdLt(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdLte(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdGt(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdGte(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdContains(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "contains".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdStartsWith(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "startsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdEndsWith(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "endsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::IdMode(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "mode".to_string(),
                        PrismaValue::Enum(value.to_string()),
                    )]),
                ),
                Self::IdNot(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressEquals(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        value
                            .map(|value| PrismaValue::String(value))
                            .unwrap_or(PrismaValue::Null),
                    )]),
                ),
                Self::AddressInVec(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::AddressNotInVec(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::AddressLt(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressLte(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressGt(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressGte(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressContains(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "contains".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressStartsWith(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "startsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressEndsWith(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "endsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AddressMode(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "mode".to_string(),
                        PrismaValue::Enum(value.to_string()),
                    )]),
                ),
                Self::AddressNot(value) => (
                    "address".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameEquals(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameInVec(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::NameNotInVec(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::NameLt(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameLte(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameGt(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameGte(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameContains(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "contains".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameStartsWith(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "startsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameEndsWith(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "endsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::NameMode(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "mode".to_string(),
                        PrismaValue::Enum(value.to_string()),
                    )]),
                ),
                Self::NameNot(value) => (
                    "name".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::CreatedAtEquals(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::DateTime(value),
                    )]),
                ),
                Self::CreatedAtInVec(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::DateTime(v))
                                .collect(),
                        ),
                    )]),
                ),
                Self::CreatedAtNotInVec(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::DateTime(v))
                                .collect(),
                        ),
                    )]),
                ),
                Self::CreatedAtLt(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::DateTime(value),
                    )]),
                ),
                Self::CreatedAtLte(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::DateTime(value),
                    )]),
                ),
                Self::CreatedAtGt(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::DateTime(value),
                    )]),
                ),
                Self::CreatedAtGte(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::DateTime(value),
                    )]),
                ),
                Self::CreatedAtNot(value) => (
                    "createdAt".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::DateTime(value),
                    )]),
                ),
                Self::MethodEquals(value) => (
                    "method".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::Enum(value.to_string()),
                    )]),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum UniqueWhereParam {
        IdEquals(String),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::IdEquals(value) => Self::IdEquals(value),
            }
        }
    }
    impl From<Operator<Self>> for WhereParam {
        fn from(op: Operator<Self>) -> Self {
            match op {
                Operator::Not(value) => Self::Not(value),
                Operator::And(value) => Self::And(value),
                Operator::Or(value) => Self::Or(value),
            }
        }
    }
    pub type UniqueArgs = prisma_client_rust::UniqueArgs<WithParam>;
    pub type ManyArgs = prisma_client_rust::ManyArgs<WhereParam, WithParam, OrderByParam, Cursor>;
    pub type Create<'a> = prisma_client_rust::Create<'a, SetParam, WithParam, Data>;
    pub type FindUnique<'a> =
        prisma_client_rust::FindUnique<'a, WhereParam, WithParam, SetParam, Data>;
    pub type FindMany<'a> = prisma_client_rust::FindMany<
        'a,
        WhereParam,
        WithParam,
        OrderByParam,
        Cursor,
        SetParam,
        Data,
    >;
    pub type FindFirst<'a> =
        prisma_client_rust::FindFirst<'a, WhereParam, WithParam, OrderByParam, Cursor, Data>;
    pub type Update<'a> = prisma_client_rust::Update<'a, WhereParam, SetParam, WithParam, Data>;
    pub type UpdateMany<'a> = prisma_client_rust::UpdateMany<'a, WhereParam, SetParam>;
    pub type Upsert<'a> = prisma_client_rust::Upsert<'a, WhereParam, SetParam, WithParam, Data>;
    pub type Delete<'a> = prisma_client_rust::Delete<'a, WhereParam, WithParam, Data>;
    pub type DeleteMany<'a> = prisma_client_rust::DeleteMany<'a, WhereParam>;
    pub struct Actions<'a> {
        pub client: &'a PrismaClient,
    }
    impl<'a> Actions<'a> {
        pub fn create(self, name: name::Set, mut _params: Vec<SetParam>) -> Create<'a> {
            _params.push(name.into());
            Create::new(
                self.client._new_query_context(),
                QueryInfo::new("Request", _outputs()),
                _params,
            )
        }
        pub fn find_unique(self, param: UniqueWhereParam) -> FindUnique<'a> {
            FindUnique::new(
                self.client._new_query_context(),
                QueryInfo::new("Request", _outputs()),
                param.into(),
            )
        }
        pub fn find_first(self, params: Vec<WhereParam>) -> FindFirst<'a> {
            FindFirst::new(
                self.client._new_query_context(),
                QueryInfo::new("Request", _outputs()),
                params,
            )
        }
        pub fn find_many(self, params: Vec<WhereParam>) -> FindMany<'a> {
            FindMany::new(
                self.client._new_query_context(),
                QueryInfo::new("Request", _outputs()),
                params,
            )
        }
        pub fn upsert(
            self,
            _where: UniqueWhereParam,
            _create: (name::Set, Vec<SetParam>),
            _update: Vec<SetParam>,
        ) -> Upsert<'a> {
            let (name, mut _params) = _create;
            _params.push(name.into());
            Upsert::new(
                self.client._new_query_context(),
                QueryInfo::new("Request", _outputs()),
                _where.into(),
                _params,
                _update,
            )
        }
    }
}
pub mod _prisma {
    use super::*;
    use prisma_client_rust::{
        queries::QueryContext,
        query_core::{QueryExecutor, QuerySchema},
        raw, ExecuteRaw, QueryRaw,
    };
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::sync::Arc;
    pub struct PrismaClient {
        executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
        query_schema: Arc<QuerySchema>,
    }
    impl fmt::Debug for PrismaClient {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PrismaClient").finish()
        }
    }
    impl PrismaClient {
        pub(super) fn _new_query_context(&self) -> QueryContext {
            QueryContext::new(&self.executor, self.query_schema.clone())
        }
        pub(super) fn _new(
            executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
            query_schema: Arc<QuerySchema>,
        ) -> Self {
            Self {
                executor,
                query_schema,
            }
        }
        pub async fn _query_raw<T: serde::de::DeserializeOwned>(
            &self,
            query: raw::Raw,
        ) -> QueryResult<Vec<T>> {
            QueryRaw::new(
                QueryContext::new(&self.executor, self.query_schema.clone()),
                query,
                DATABASE_STR,
            )
            .exec()
            .await
        }
        pub async fn _execute_raw(&self, query: raw::Raw) -> QueryResult<i64> {
            ExecuteRaw::new(
                QueryContext::new(&self.executor, self.query_schema.clone()),
                query,
                DATABASE_STR,
            )
            .exec()
            .await
        }
        pub fn request(&self) -> request::Actions {
            request::Actions { client: &self }
        }
    }
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum RequestScalarFieldEnum {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "address")]
        Address,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdAt")]
        CreatedAt,
        #[serde(rename = "method")]
        Method,
    }
    impl ToString for RequestScalarFieldEnum {
        fn to_string(&self) -> String {
            match self {
                Self::Id => "id".to_string(),
                Self::Address => "address".to_string(),
                Self::Name => "name".to_string(),
                Self::CreatedAt => "createdAt".to_string(),
                Self::Method => "method".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl ToString for SortOrder {
        fn to_string(&self) -> String {
            match self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum QueryMode {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "insensitive")]
        Insensitive,
    }
    impl ToString for QueryMode {
        fn to_string(&self) -> String {
            match self {
                Self::Default => "default".to_string(),
                Self::Insensitive => "insensitive".to_string(),
            }
        }
    }
}
pub use _prisma::PrismaClient;
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
}
impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET".to_string(),
            Self::Post => "POST".to_string(),
            Self::Put => "PUT".to_string(),
            Self::Patch => "PATCH".to_string(),
            Self::Delete => "DELETE".to_string(),
        }
    }
}
