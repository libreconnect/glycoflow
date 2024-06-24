use std::sync::Arc;

use futures_lite::StreamExt;
use lapin::{
    options::{BasicConsumeOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable},
    Channel,
};
use tracing::info;

pub async fn listen_queue(queue_name: &str, channel: Arc<Channel>) {
    let mut arguments = FieldTable::default();

    arguments.insert(
        "x-queue-type".into(),
        AMQPValue::LongString("quorum".into()),
    );
    let queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            arguments,
        )
        .await;

    if queue.is_err() {
        println!("Error declaring queue: {:?}", queue.err());
        return;
    }

    let mut consumer = channel
        .basic_consume(
            queue_name,
            "my_consumer",
            BasicConsumeOptions {
                no_ack: false,
                ..Default::default()
            },
            Default::default(),
        )
        .await
        .unwrap();

    info!("Listening to queue: {}", queue_name);

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.unwrap();
        let message = &delivery.data;

        let message = std::str::from_utf8(message).unwrap();

        info!("Received message: {}", message);
    }
}
