use redis::{streams::StreamReadReply, Commands};
use serde_redis_stream_interface::{Foobar, RedisStreamSerializable};

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(main_async());
}

async fn main_async() {
    let redis_client = redis::Client::open("redis://127.0.0.1/0").unwrap();

    // let foobar = Foobar {
    //     name: "original".to_string(),
    //     age: 100,
    // };

    // let mut redis_connection = redis_client.get_connection().unwrap();
    // let a = foobar.redis_serialize("gambiarra", "*");
    // a.execute(&mut redis_connection);

    // let mut redis_connection = redis_client.get_connection().unwrap();

    let mut redis_connection = redis_client.get_connection().unwrap();
    let a: StreamReadReply = redis_connection.xread(&["gambiarra"], &["0"]).unwrap();

    let foobar = Foobar::redis_deserialize(a.keys.first().unwrap().clone());

    println!("Resultado: {:#?}", foobar);
}
