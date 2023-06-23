use serde_redis_stream_derive::RedisStreamSerialize;
use serde_redis_stream_interface::RedisStreamSerializable;

#[derive(RedisStreamSerialize)]
struct Foobar {
    name: Option<String>,
    #[serialize = "bincode"]
    age: Option<i64>,
    #[serialize = "json"]
    lat: Option<f64>,
}

fn main() {
    let foobar = Foobar {
        name: Some("foobar".to_string()),
        age: Some(10),
        lat: Some(10.0),
    };

    foobar.redis_serialize("foobar", "*").unwrap();
}
