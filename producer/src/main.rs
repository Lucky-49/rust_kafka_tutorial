use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let topic_name = "test-topic";
    create_topic(topic_name, 3, 3).await;

    let producer = create_producer();
    let mut counter = 0;

    loop {
        counter += 1;
        produce(
            &producer,
            topic_name,
            format!("Test message {}", counter).as_str(),
        )
        .await;
        sleep(Duration::from_secs(1));
    }
}

fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set(
            "bootstrap.servers",
            "host.docker.internal:9092, host.docker.internal:9093, host.docker.internal:9094",
        )
        .set("acks", "all")
        .set("retries", "5")
        .set("reconnect.backoff.ms", "500")
        .set("reconnect.backoff.max.ms", "5000")
        .set("security.protocol", "ssl")
        //.set("ssl.ca.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/chain.crt")
        .set("ssl.ca.location", "/app/producer/chain.crt")
        //.set("ssl.certificate.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/producer.crt")
        .set("ssl.certificate.location", "/app/producer/producer.crt")
        //.set("ssl.key.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/producer.key")
        .set("ssl.key.location", "/app/producer/producer.key")
        .set("ssl.key.password", "changeit")
        .set("ssl.endpoint.identification.algorithm", "none")
        .set("debug", "security,broker,protocol")
        .create()
        .expect("Ошибка в create_producer")
}

async fn produce(producer: &FutureProducer, topic: &str, msg: &str) {
    let record = FutureRecord::to(topic).payload(msg).key("my_key");

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(report) => println!("Отправлено сообщение {:?}", report),
        Err(e) => println!("Ошибка отправки {:?}", e),
    }
}

async fn create_topic(topic_name: &str, quantity_partitions: i32, replication_factor: i32) {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set(
            "bootstrap.servers",
            "host.docker.internal:9092, host.docker.internal:9093, host.docker.internal:9094",
        )
        .set("security.protocol", "ssl")
        //.set("ssl.ca.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/chain.crt")
        .set("ssl.ca.location", "/app/producer/chain.crt")
        //.set("ssl.certificate.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/producer.crt")
        .set("ssl.certificate.location", "/app/producer/producer.crt")
        //.set("ssl.key.location", "C:/Users/Lucky/RustroverProjects/test_kafka/secrets/producer/producer.key")
        .set("ssl.key.location", "/app/producer/producer.key")
        .set("ssl.key.password", "changeit")
        .set("ssl.endpoint.identification.algorithm", "none")
        .set("debug", "security,broker,protocol")
        .create()
        .expect("Не удалось создать админ клиента");

    let topic = NewTopic::new(
        topic_name,
        quantity_partitions,
        TopicReplication::Fixed(replication_factor),
    );

    match admin_client
        .create_topics(&[topic], &AdminOptions::new())
        .await
    {
        Ok(_) => println!("Топик {} успешно создан", topic_name),
        Err(e) => println!("Ошибка создания топика {}: {:?}", topic_name, e),
    }
}
