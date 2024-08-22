use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub fn create() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");

    let producer: FutureProducer = config
        .create()
        .expect("Не запустился продюсер");

    producer
}

pub async fn produce(future_produce: FutureProducer, msg: String) {
    let record = FutureRecord::to("test-topic")
        .payload(msg.as_str())
        .key("Test-key");

    let status_delivery = future_produce
        .send(record, Timeout::After(Duration::from_secs(1)))
        .await;

    match status_delivery {
        Ok(report) => println!("Отправлено сообщение {:?}", report),
        Err(e) => println!("Ошибка отправки {:?}", e)
    }
}