use redis::{streams::StreamReadReply, Commands};

use serde_redis_stream_derive::RedisStreamSerialize;
use serde_redis_stream_interface::RedisStreamSerializable;

#[test]
fn it_works_option_some() {
    #[derive(Debug, RedisStreamSerialize, PartialEq, Eq)]
    struct Foobar {
        name: Option<i64>,
        #[serialize = "bincode"]
        age: Option<i64>,
        #[serialize = "json"]
        foobar: Option<i64>,
    }

    let redis_client = redis::Client::open("redis://127.0.0.1/0").unwrap();
    let mut redis_connection = redis_client.get_connection().unwrap();

    let _: () = redis_connection.del("gambiarra7").unwrap();

    let foobar = Foobar {
        name: Some(1),
        age: Some(1),
        foobar: Some(1),
    };

    let a = foobar.redis_serialize("gambiarra7", "*").unwrap();

    a.execute(&mut redis_connection);

    let a: StreamReadReply = redis_connection.xread(&["gambiarra7"], &["0"]).unwrap();
    let foobar_result = Foobar::redis_deserialize(a.keys.first().unwrap().clone()).unwrap();

    assert_eq!(foobar, foobar_result);
}

#[test]
fn it_works_option_none() {
    #[derive(Debug, RedisStreamSerialize, PartialEq, Eq)]
    struct Foobar {
        name: Option<String>,
        #[serialize = "bincode"]
        age: Option<i64>,
        #[serialize = "json"]
        foobar: Option<bool>,
    }

    let redis_client = redis::Client::open("redis://127.0.0.1/0").unwrap();
    let mut redis_connection = redis_client.get_connection().unwrap();

    let _: () = redis_connection.del("gambiarra10").unwrap();

    let foobar = Foobar {
        name: None,
        age: None,
        foobar: None,
    };

    let a = foobar.redis_serialize("gambiarra10", "*").unwrap();

    a.execute(&mut redis_connection);

    let a: StreamReadReply = redis_connection.xread(&["gambiarra10"], &["0"]).unwrap();
    let foobar_result = Foobar::redis_deserialize(a.keys.first().unwrap().clone()).unwrap();

    assert_eq!(foobar, foobar_result);
}

#[test]
fn it_works() {
    #[derive(Debug, RedisStreamSerialize, PartialEq, Eq)]
    struct Foobar {
        name: String,
        age: i64,
        foobar: bool,
    }

    let redis_client = redis::Client::open("redis://127.0.0.1/0").unwrap();
    let mut redis_connection = redis_client.get_connection().unwrap();

    let _: () = redis_connection.del("gambiarra0").unwrap();

    let foobar = Foobar {
        name: "original".to_string(),
        age: 100,
        foobar: true,
    };

    let a = foobar.redis_serialize("gambiarra0", "*").unwrap();

    a.execute(&mut redis_connection);

    let a: StreamReadReply = redis_connection.xread(&["gambiarra0"], &["0"]).unwrap();
    let foobar_result = Foobar::redis_deserialize(a.keys.first().unwrap().clone()).unwrap();

    assert_eq!(foobar, foobar_result);
}

#[test]
fn it_works_bincode() {
    #[derive(Debug, RedisStreamSerialize, PartialEq, Eq)]
    struct Foobar {
        name: String,
        #[serialize = "bincode"]
        age: i64,
        foobar: bool,
    }

    let redis_client = redis::Client::open("redis://127.0.0.1/0").unwrap();
    let mut redis_connection = redis_client.get_connection().unwrap();

    let _: () = redis_connection.del("gambiarra1").unwrap();

    let foobar = Foobar {
        name: "original".to_string(),
        age: 100,
        foobar: true,
    };

    let a = foobar.redis_serialize("gambiarra1", "*").unwrap();
    a.execute(&mut redis_connection);

    let a: StreamReadReply = redis_connection.xread(&["gambiarra1"], &["0"]).unwrap();

    let foobar_result = Foobar::redis_deserialize(a.keys.first().unwrap().clone()).unwrap();
    assert_eq!(foobar, foobar_result);
}

#[test]

fn it_works_json() {
    #[derive(Debug, RedisStreamSerialize, PartialEq, Eq)]
    struct Foobar {
        name: String,
        #[serialize = "json"]
        age: i64,
        foobar: bool,
    }

    let redis_client = redis::Client::open("redis://127.0.0.1/0").unwrap();
    let mut redis_connection = redis_client.get_connection().unwrap();

    let _: () = redis_connection.del("gambiarra2").unwrap();

    let foobar = Foobar {
        name: "original".to_string(),
        age: 100,
        foobar: true,
    };

    let a = foobar.redis_serialize("gambiarra2", "*").unwrap();
    a.execute(&mut redis_connection);

    let a: StreamReadReply = redis_connection.xread(&["gambiarra2"], &["0"]).unwrap();

    let foobar_result = Foobar::redis_deserialize(a.keys.first().unwrap().clone()).unwrap();
    assert_eq!(foobar, foobar_result);
}
