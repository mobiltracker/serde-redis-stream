use redis::{streams::StreamKey, FromRedisValue};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisStreamDeriveError {
    #[error("Field in hash maps wasn't found: `{0}`")]
    MissingFieldFromHashMap(String),
}

#[derive(Debug)]
pub struct Foobar {
    pub name: String, // ToRedisArgs
    pub age: i64,
}

pub trait RedisStreamSerializable: Sized {
    fn redis_serialize(&self, stream_name: &str, id: &str) -> redis::Cmd;
    fn redis_deserialize(value: StreamKey) -> Result<Self, RedisStreamDeriveError>;
}

impl RedisStreamSerializable for Foobar {
    fn redis_serialize(&self, stream_name: &str, id: &str) -> redis::Cmd {
        let mut cmd: redis::Cmd = redis::cmd("XADD");

        cmd.arg(stream_name)
            .arg(id)
            .arg("name")
            .arg(&self.name)
            .arg("age")
            .arg(self.age);

        cmd
    }

    fn redis_deserialize(value: StreamKey) -> Result<Self, RedisStreamDeriveError> {
        let ids = value.ids;

        let map = &ids.first().unwrap().map;

        let name: &redis::Value = map
            .get("name")
            .ok_or_else(|| RedisStreamDeriveError::MissingFieldFromHashMap(String::from("name")))?;
        let name: String =
            <String as redis::FromRedisValue>::from_redis_value(name).expect("TODO DESERIALIZE");

        let age = map
            .get("age")
            .map(i64::from_redis_value)
            .transpose()
            .expect("TODO - DESERIALIZATION")
            .expect("TODO - NO STRING");

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
