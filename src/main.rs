use crate::producer::produce;

mod producer;
mod consumer;

#[tokio::main]
async fn main() {
    let producer = producer::create();
    produce(producer, "Тестовое сообщение кафка".to_string()).await;
    consumer::start().await;
}
