use redis::streams::StreamKey;

pub struct Foobar {
    pub name: String, // ToRedisArgs
    pub age: i64,
}

pub trait RedisStreamSerializable {
    fn redis_serialize(&self, stream_name: &str, id: &str) -> redis::Cmd;
    fn redis_deserialize(value: StreamKey) -> Self;
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

    fn redis_deserialize(value: StreamKey) -> Self {
        let ids = value.ids;

        let map = &ids.first().unwrap().map;

        let name = match map.get("name").unwrap() {
            redis::Value::Int(data) => data,
            _ => unimplemented!(),
        };
        let age = match map.get("age").unwrap() {
            redis::Value::Int(data) => *data,
            _ => unimplemented!(),
        };

        Foobar {
            name: name.to_string(),
            age,
        }
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
