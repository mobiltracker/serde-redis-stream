use serde_redis_stream::RedisStreamSerialize;
use serde_redis_stream_interface::RedisStreamSerializable;

#[derive(RedisStreamSerialize)]
struct Foobar {
    name: String,
    #[serialize = "bincode"]
    age: i64,
    #[serialize = "json"]
    lat: f64,
}

fn main() {
    let foobar = Foobar {
        name: "foobar".to_string(),
        age: 10,
        lat: 10.0,
    };

    foobar.redis_serialize("foobar", "*");
}
