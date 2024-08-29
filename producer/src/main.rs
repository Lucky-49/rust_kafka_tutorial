use std::thread::sleep;
use std::time::Duration;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[tokio::main]
async fn main() {
    let topic_name = "test-topic-4";
    create_topic(topic_name, 3, 3).await;

    let producer = create_producer();
    let mut counter = 0;

    loop {
        counter += 1;
        produce(&producer, topic_name, format!("Test message {}", counter).as_str()).await;
        sleep(Duration::from_secs(2));
    }
}

pub fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092, localhost:29093, localhost:29094")
        .set("acks", "all")
        .set("retries", "5")
        .set("reconnect.backoff.ms", "500")
        .set("reconnect.backoff.max.ms", "5000")
        .create()
        .expect("Ошибка в create_producer")
}

pub async fn produce(producer: &FutureProducer, topic: &str, msg: &str) {
        let record = FutureRecord::to(topic)
            .payload(msg)
            .key("my_key");

        match producer.send(record, Duration::from_secs(0)).await {
            Ok(report) => println!("Отправлено сообщение {:?}", report),
            Err(e) => println!("Ошибка отправки {:?}", e)
        }
}

async fn create_topic(topic_name: &str, quantity_partitions: i32, replication_factor: i32) {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092,localhost:29093,localhost:29094")
        .create()
        .expect("Не удалось создать админ клиента");

    let topic = NewTopic::new(topic_name, quantity_partitions, TopicReplication::Fixed(replication_factor));

    match admin_client.create_topics(&[topic], &AdminOptions::new()).await {
        Ok(_) => println!("Топик {} успешно создан", topic_name),
        Err(e) => println!("Ошибка создания топика {}: {:?}", topic_name, e),
    }
}