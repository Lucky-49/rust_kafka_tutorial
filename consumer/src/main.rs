use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let consumer = create_consumer();
    println!("Консьюмер создан, начинаю получение сообщений...");
    consume(&consumer).await;
}

fn create_consumer() -> StreamConsumer {
    ClientConfig::new()
        .set(
            "bootstrap.servers",
            "host.docker.internal:9092, host.docker.internal:9093, host.docker.internal:9094",
        )
        .set("group.id", "test-group")
        .set("auto.offset.reset", "earliest")
        .set("security.protocol", "ssl")
        //.set("ssl.ca.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/chain.crt")
        .set("ssl.ca.location", "/app/consumer/chain.crt")
        //.set("ssl.certificate.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/producer.crt")
        .set("ssl.certificate.location", "/app/consumer/consumer.crt")
        //.set("ssl.key.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/producer.key")
        .set("ssl.key.location", "/app/consumer/consumer.key")
        .set("ssl.key.password", "changeit")
        .set("ssl.endpoint.identification.algorithm", "none")
        .set("debug", "security,broker,protocol")
        .create()
        .expect("Ошибка в create_consumer")
}

async fn consume(consumer: &StreamConsumer) {
    let topic_name = "test-topic";
    println!("Подписываемся на топик: {}", topic_name);

    if let Err(e) = consumer.subscribe(&[topic_name]) {
        eprintln!("Ошибка подписки на топик {}: {:?}", topic_name, e);
        return;
    } else {
        println!("Успешная подписка на топик {}", topic_name);
    }

    let mut message_stream = consumer.stream();
println!("продолжаем1");
    while let Some(message) = message_stream.next().await {
        println!("продолжаем2");

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
    println!("Поток сообщений завершен");
}
