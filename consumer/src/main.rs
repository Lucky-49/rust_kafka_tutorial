use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let consumer = create_consumer();
    consume(&consumer).await;
}

pub async fn start_consumer() {
    let consumer = create_consumer();
    consume(&consumer).await
}

fn create_consumer() -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092, localhost:29093, localhost:29094")
        .set("group.id", "test-group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Ошибка в create_consumer")
}

async fn consume(consumer: &StreamConsumer) {
    let topic_name = "test-topic-4";
    consumer.subscribe(&[topic_name]).expect("не удалось подписаться на топик");

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload_view::<str>() {
                    match payload {
                        Ok(text) => println!("Консьюмер получил сообщение {}", text),
                        Err(e) => println!("Ошибка десириализации сообщения консьюмером {:?}", e),
                    }
                } else {
                    println!("Пустой payload");
                }
                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
            Err(e) => println!("Ошибка получения сообщения {:?}", e),
        }
    }
}