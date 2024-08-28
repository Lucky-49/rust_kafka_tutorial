use std::thread::sleep;
use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

#[tokio::main]
async fn main() {
    let producer = create_producer();

    loop {
        produce(&producer, "test-topic-3", "Test message").await;
        sleep(Duration::from_secs(1));
    }
}

pub fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092, localhost:29093, localhost:29094")
        .create()
        .expect("Ошибка в create_producer")
}

pub async fn produce(producer: &FutureProducer, topic: &str, msg: &str) {
        let record = FutureRecord::to(topic)
            .payload(msg)
            .key("Test-key");

        match producer.send(record, Duration::from_secs(0)).await {
            Ok(report) => println!("Отправлено сообщение {:?}", report),
            Err(e) => println!("Ошибка отправки {:?}", e)
        }

}