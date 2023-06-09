use redis::{streams::StreamReadOptions, Pipeline};

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(main_async());
}

async fn main_async() {
    // let foobar = Foobar {
    //     age: 23,
    //     name: "amanda".to_string(),
    // };

    let redis_client = redis::Client::open("redis://127.0.0.1").unwrap();
    let mut redis_connection = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .unwrap();

    // match foobar
    //     .redis_serialize("foobar", "*")
    //     .query_async::<redis::aio::MultiplexedConnection, ()>(&mut redis_connection)
    //     .await
    // {
    //     Ok(_) => (),
    //     Err(_) => println!("Erro complicado"),
    // };

    let mut pipeline = Pipeline::new();

    let opts = StreamReadOptions::default().count(1);
    pipeline.xread_options(&["foobar"], &["0"], &opts);

    match pipeline
        .query_async::<redis::aio::MultiplexedConnection, redis::Value>(&mut redis_connection)
        .await
    {
        Ok(value) => {
            match value {
                redis::Value::Nil => todo!(),
                redis::Value::Int(_) => todo!(),
                redis::Value::Data(_) => todo!(),
                redis::Value::Bulk(values) => {
                    for value in values {
                        println!("################################\n");
                        match value {
                            redis::Value::Nil => todo!(),
                            redis::Value::Int(_) => todo!(),
                            redis::Value::Data(_) => todo!(),
                            redis::Value::Bulk(values) => {
                                for value in values {
                                    println!("-----------------------------------\n");
                                    match value {
                                        redis::Value::Nil => todo!(),
                                        redis::Value::Int(_) => todo!(),
                                        redis::Value::Data(_) => todo!(),
                                        redis::Value::Bulk(values) => {
                                            for value in values {
                                                println!("***********************************\n");
                                                match value {
                                                    redis::Value::Nil => todo!(),
                                                    redis::Value::Int(_) => todo!(),
                                                    redis::Value::Data(value) => {
                                                        match std::str::from_utf8(&value) {
                                                            Ok(value) => {
                                                                println!("Resultado Primeiro Nível: {:#?}", value)
                                                            }
                                                            Err(_) => todo!(),
                                                        }
                                                    }
                                                    redis::Value::Bulk(values) => {
                                                        for value in values {
                                                            match value {
                                                                redis::Value::Nil => todo!(),
                                                                redis::Value::Int(_) => todo!(),
                                                                redis::Value::Data(_) => todo!(),
                                                                redis::Value::Bulk(values) => {
                                                                    for value in values {
                                                                        match value {
                                                                        redis::Value::Nil => todo!(),
                                                                        redis::Value::Int(_) => todo!(),
                                                                        redis::Value::Data(value) => {
                                                                            match std::str::from_utf8(&value) {
                                                                                Ok(value) => {
                                                                                    println!("Resultado Segundo Nível: {:#?}", value)
                                                                                }
                                                                                Err(_) => todo!(),
                                                                            }
                                                                        }
                                                                        redis::Value::Bulk(values) => {
                                                                            for value in values {
                                                                                match value {
                                                                                    redis::Value::Nil => todo!(),
                                                                                    redis::Value::Int(_) => todo!(),
                                                                                    redis::Value::Data(value) => {
                                                                                        match std::str::from_utf8(&value) {
                                                                                            Ok(value) => {
                                                                                                println!("Resultado Terceiro Nível: {:#?}", value)
                                                                                            }
                                                                                            Err(_) => todo!(),
                                                                                        }
                                                                                    }
                                                                                    redis::Value::Bulk(values) => {
                                                                                        for value in values {
                                                                                            match value {
                                                                                                redis::Value::Nil => todo!(),
                                                                                                redis::Value::Int(_) => todo!(),
                                                                                                redis::Value::Data(value) => {
                                                                                                    match std::str::from_utf8(&value) {
                                                                                                        Ok(value) => {
                                                                                                            println!("Resultado Quarto Nível: {:#?}", value)
                                                                                                        }
                                                                                                        Err(_) => todo!(),
                                                                                                    }
                                                                                                }
                                                                                                redis::Value::Bulk(_) => todo!(),
                                                                                                redis::Value::Status(_) => todo!(),
                                                                                                redis::Value::Okay => todo!(),
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    redis::Value::Status(_) => todo!(),
                                                                                    redis::Value::Okay => todo!(),
                                                                                }
                                                                            }
                                                                        }
                                                                        redis::Value::Status(_) => todo!(),
                                                                        redis::Value::Okay => todo!(),
                                                                    }
                                                                    }
                                                                }
                                                                redis::Value::Status(_) => todo!(),
                                                                redis::Value::Okay => todo!(),
                                                            }
                                                        }
                                                    }
                                                    redis::Value::Status(_) => todo!(),
                                                    redis::Value::Okay => todo!(),
                                                }
                                            }
                                        }
                                        redis::Value::Status(_) => todo!(),
                                        redis::Value::Okay => todo!(),
                                    }
                                }
                            }
                            redis::Value::Status(_) => todo!(),
                            redis::Value::Okay => todo!(),
                        }
                    }
                }
                redis::Value::Status(_) => todo!(),
                redis::Value::Okay => todo!(),
            }
        }
        Err(err) => println!("Erro: {:#?}", err),
    };
}
