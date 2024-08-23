use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub fn create() -> FutureProducer {
    println!("старт продюсер креате");

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:29092, localhost:29093, localhost:29094");

    let producer: FutureProducer = config
        .create()
        .expect("Не запустился продюсер");

    producer
}

pub async fn produce(future_produce: FutureProducer, msg: String) {
    println!("старт продюсер продюсе");

    let record = FutureRecord::to("test-topic")
        .payload(msg.as_str())
        .key("Test-key");

    println!("прошли record");

    let status_delivery = future_produce
        .send(record, Timeout::After(Duration::from_secs(10)))
        .await;

    println!("прошил статус деливери");

    match status_delivery {
        Ok(report) => println!("Отправлено сообщение {:?}", report),
        Err(e) => println!("Ошибка отправки {:?}", e)
    }
}