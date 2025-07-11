use derive_more::{AsMut, AsRef, Deref};
use prost::{
    bytes::{Buf, BufMut},
    encoding::{skip_field, uint64, DecodeContext, WireType},
    DecodeError, Message,
};
use sea_orm::entity::prelude::*;
// use sea_orm::sea_query;
use sea_orm::TryGetError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    ParseError(String),
}

#[derive(
    AsRef, AsMut, Clone, Copy, Debug, Default, Deref, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct ID(pub Uuid);

#[derive(
    AsMut, AsRef, Clone, Copy, Debug, Default, Deref, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct SqliteID(pub Uuid);

impl ID {
    pub fn new() -> ID {
        ID(Uuid::now_v7())
    }

    pub fn nil() -> ID {
        ID(Uuid::nil())
    }
    pub fn parse(s: &str) -> Result<ID, Error> {
        let id = Uuid::parse_str(s).map_err(|_| Error::ParseError("bad id".to_string()))?;
        Ok(ID(id))
    }
}
impl SqliteID {
    pub fn new() -> SqliteID {
        SqliteID(Uuid::now_v7())
    }

    pub fn nil() -> SqliteID {
        SqliteID(Uuid::nil())
    }
    pub fn parse(s: &str) -> Result<SqliteID, Error> {
        let id = Uuid::parse_str(s).map_err(|_| Error::ParseError("bad id".to_string()))?;
        Ok(SqliteID(id))
    }
}

impl From<ID> for Uuid {
    fn from(id: ID) -> Self {
        id.0
    }
}

impl From<Uuid> for ID {
    fn from(id: Uuid) -> Self {
        ID(id)
    }
}

impl sea_orm::TryFromU64 for ID {
    fn try_from_u64(_n: u64) -> Result<Self, sea_orm::DbErr> {
        Err(sea_orm::DbErr::ConvertFromU64("ID"))
    }
}
impl sea_orm::TryFromU64 for SqliteID {
    fn try_from_u64(_n: u64) -> Result<Self, sea_orm::DbErr> {
        Err(sea_orm::DbErr::ConvertFromU64("SqliteID"))
    }
}

impl From<SqliteID> for sea_orm::Value {
    fn from(source: SqliteID) -> Self {
        let v: Vec<u8> = source.0.into();
        sea_orm::Value::Bytes(Some(Box::new(v)))
    }
}

impl From<ID> for sea_orm::Value {
    fn from(source: ID) -> Self {
        sea_orm::Value::Uuid(Some(Box::new(source.0)))
    }
}

impl sea_orm::TryGetable for SqliteID {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        let val: Vec<u8> = res.try_get(pre, col).map_err(sea_orm::TryGetError::DbErr)?;
        let val = Uuid::from_slice(val.as_slice())
            .map_err(|e| sea_orm::TryGetError::DbErr(DbErr::Custom(e.to_string())))?;
        Ok(SqliteID(val))
    }

    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let val: Vec<u8> = res.try_get_by(index).map_err(sea_orm::TryGetError::DbErr)?;
        let v = Uuid::from_slice(val.as_slice())
            .map_err(|e| sea_orm::TryGetError::DbErr(DbErr::Custom(e.to_string())))?;
        Ok(SqliteID(v))
    }
}

impl sea_orm::TryGetable for ID {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        let val = <Uuid as sea_orm::TryGetable>::try_get(res, pre, col)?;
        Ok(ID(val))
    }

    fn try_get_by<I: sea_orm::ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let val = <Uuid as sea_orm::TryGetable>::try_get_by(res, index)?;
        Ok(ID(val))
    }
}

impl sea_orm::sea_query::ValueType for SqliteID {
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
    fn type_name() -> String {
        "SqliteID".to_string()
    }
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::Bytes(Some(x)) => Ok(SqliteID(
                Uuid::from_slice(x.as_slice()).map_err(|_| sea_orm::sea_query::ValueTypeErr)?,
            )),
            sea_orm::Value::String(Some(s)) => {
                let id = Uuid::parse_str(&s).map_err(|_| sea_orm::sea_query::ValueTypeErr)?;
                Ok(SqliteID(id))
            }
            sea_orm::Value::Uuid(Some(id)) => Ok(SqliteID(*id)),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }
    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Blob
    }
}
impl sea_orm::sea_query::ValueType for ID {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        println!("try_from: {:?}", v);
        match v {
            sea_orm::Value::Bytes(Some(x)) => Ok(ID(
                Uuid::from_slice(x.as_slice()).map_err(|_| sea_orm::sea_query::ValueTypeErr)?
            )),
            sea_orm::Value::String(Some(s)) => {
                let id = Uuid::parse_str(&s).map_err(|_| sea_orm::sea_query::ValueTypeErr)?;
                Ok(ID(id))
            }
            sea_orm::Value::Uuid(Some(id)) => Ok(ID(*id)),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "ID".to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Uuid
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Uuid
    }
}

impl sea_orm::sea_query::Nullable for SqliteID {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Bytes(None)
    }
}

impl sea_orm::sea_query::Nullable for ID {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Uuid(None)
    }
}

const HIGH_TAG: u32 = 1;
const LOW_TAG: u32 = 2;

impl Message for ID {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        let (high, low) = self.0.as_u64_pair();
        uint64::encode(HIGH_TAG, &high, buf);
        uint64::encode(LOW_TAG, &low, buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        match tag {
            HIGH_TAG => {
                let (mut high, low) = self.0.as_u64_pair();
                uint64::merge(wire_type, &mut high, buf, ctx)?;
                self.0 = Uuid::from_u64_pair(high, low);
                Ok(())
            }
            LOW_TAG => {
                let (high, mut low) = self.0.as_u64_pair();
                uint64::merge(wire_type, &mut low, buf, ctx)?;
                self.0 = Uuid::from_u64_pair(high, low);
                Ok(())
            }
            _ => skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        let (high, low) = self.0.as_u64_pair();
        uint64::encoded_len(HIGH_TAG, &high) + uint64::encoded_len(LOW_TAG, &low)
    }

    fn clear(&mut self) {
        self.0 = Uuid::nil();
    }
}

impl Message for SqliteID {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        let (high, low) = self.0.as_u64_pair();
        uint64::encode(HIGH_TAG, &high, buf);
        uint64::encode(LOW_TAG, &low, buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        match tag {
            HIGH_TAG => {
                let (mut high, low) = self.0.as_u64_pair();
                uint64::merge(wire_type, &mut high, buf, ctx)?;
                self.0 = Uuid::from_u64_pair(high, low);
                Ok(())
            }
            LOW_TAG => {
                let (high, mut low) = self.0.as_u64_pair();
                uint64::merge(wire_type, &mut low, buf, ctx)?;
                self.0 = Uuid::from_u64_pair(high, low);
                Ok(())
            }
            _ => skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        let (high, low) = self.0.as_u64_pair();
        uint64::encoded_len(HIGH_TAG, &high) + uint64::encoded_len(LOW_TAG, &low)
    }

    fn clear(&mut self) {
        self.0 = Uuid::nil();
    }
}
