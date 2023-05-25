use redis::streams::StreamKey;

struct Foobar {
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
        todo!()
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
