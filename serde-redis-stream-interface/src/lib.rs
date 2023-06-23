use redis::streams::StreamKey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisStreamDeriveError {
    #[error("Missing field in StreamKey(HashMap): `{0}`")]
    MissingFieldFromHashMap(String),
    #[error("Error in deserialization from redis value in: `{0}`")]
    DeserializationErrorFromRedisValue(String),
    #[error("Error in deserialization to Vec<u8> from redis value in: `{0}`")]
    DeserializationErrorFromRedisValueToVecU8(String),
    #[error("Error in deserialization to String from redis value in: `{0}`")]
    DeserializationErrorFromRedisValueToString(String),
    #[error("Error in deserialization from bincode value in: `{0}`")]
    DeserializationErrorFromBincode(String),
    #[error("Error in deserialization from JSON value in: `{0}`")]
    DeserializationErrorFromJSON(String),
    #[error("Error in serialization to bincode in: `{0}`")]
    SerializationErrorToBincode(String),
    #[error("Error in serialization to JSON in: `{0}`")]
    SerializationErrorToJSON(String),
    #[error("Item on StreamKey is invalid.")]
    InvalidItemOnStreamKey,
}

#[derive(Debug)]
pub struct Foobar {
    pub name: String, // ToRedisArgs
    pub age: i64,
}

pub trait RedisStreamSerializable: Sized {
    fn redis_serialize(
        &self,
        stream_name: &str,
        id: &str,
    ) -> Result<redis::Cmd, RedisStreamDeriveError>;
    fn redis_deserialize(value: StreamKey) -> Result<Self, RedisStreamDeriveError>;
}

impl RedisStreamSerializable for Foobar {
    fn redis_serialize(
        &self,
        stream_name: &str,
        id: &str,
    ) -> Result<redis::Cmd, RedisStreamDeriveError> {
        let mut cmd: redis::Cmd = redis::cmd("XADD");

        cmd.arg(stream_name)
            .arg(id)
            .arg("name")
            .arg(&self.name)
            .arg("age")
            .arg(self.age);

        Ok(cmd)
    }

    fn redis_deserialize(value: StreamKey) -> Result<Self, RedisStreamDeriveError> {
        let ids = value.ids;

        let map = &ids
            .first()
            .ok_or(RedisStreamDeriveError::InvalidItemOnStreamKey)?
            .map;

        let name: &redis::Value = map
            .get("name")
            .ok_or_else(|| RedisStreamDeriveError::MissingFieldFromHashMap(String::from("name")))?;
        let name: String = <String as redis::FromRedisValue>::from_redis_value(name)
            .map_err(|_| RedisStreamDeriveError::MissingFieldFromHashMap(String::from("name")))?;

        let age = map
            .get("age")
            .ok_or_else(|| RedisStreamDeriveError::MissingFieldFromHashMap(String::from("age")))?;
        let age: i64 = <i64 as redis::FromRedisValue>::from_redis_value(age)
            .map_err(|_| RedisStreamDeriveError::MissingFieldFromHashMap(String::from("age")))?;

        Ok(Foobar { name, age })
    }
}

#[cfg(test)]
mod tests {

    use crate::{Foobar, RedisStreamSerializable};

    fn _it_compiles() {
        let foobar = Foobar {
            age: 18,
            name: "jose".to_string(),
        };

        let _cmd = foobar.redis_serialize("foobar", "*");
    }
}
