use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};

pub async fn start() {
    println!("страт копсьюмер старт");

    let consumer = create();
    consume(consumer).await
}

fn create() -> StreamConsumer {
    println!("старт консьюмер креате");

    let mut  binding = ClientConfig::new();
    let config = binding
        .set("bootstrap.servers", "localhost:29092, localhost:29093, localhost:29094")
        .set("auto.offset.reset", "earliest")
        .set("group.id", "test-group")
        .set("socket.timeout.ms", "4000");

    let consumer = config.create().expect("Ошибка создания консьюмера");

    consumer
}

async fn consume(consumer: StreamConsumer) {
    println!("старт консьюмер консум");

    consumer.subscribe(&["test-topic"]).expect("не удалось подписаться на топик");

    loop {
        match consumer.recv().await {
            Err(e) => println!("{:?}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("None message"),
                    Some(Ok(msg)) => println!("Сообщение консьюмера: {}",msg),
                    Some(Err(e)) => println!("Ошибка парсинга: {}", e)
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap();
            }
        }
    }
}