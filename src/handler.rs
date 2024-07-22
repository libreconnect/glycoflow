use std::sync::Arc;

use futures_lite::StreamExt;
use lapin::{
  Channel,
  options::{BasicConsumeOptions, QueueDeclareOptions},
  types::{AMQPValue, FieldTable},
};
use sqlx::PgPool;
use tracing::info;

use crate::{
  Encryptor,
  handlers::{
    command_create_access_token::{create_access_token, MessageCreateAccessToken},
    command_fetch_once::{fetch_once, MessageFetchOnce},
  },
};

pub async fn listen_queue(
  queue_name: &str,
  channel: Arc<Channel>,
  pool: Arc<PgPool>,
  encryptor: Arc<Encryptor>,
) -> Result<(), Box<dyn std::error::Error>> {
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

  if let Err(err) = queue {
    println!("Error declaring queue: {:?}", err);
    return Err(Box::new(err));
  }

  let mut consumer = channel
      .basic_consume(
        queue_name,
        queue_name,
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

    match queue_name {
      "glycoflow_register_access_token" => {
        let message = serde_json::from_str::<MessageCreateAccessToken>(message)?;
        create_access_token(
          message,
          channel.clone(),
          delivery,
          pool.clone(),
          encryptor.clone(),
        )
            .await?;
      }
      "glycoflow_command_fetch_once" => {
        let message = serde_json::from_str::<MessageFetchOnce>(message)?;
        fetch_once(
          message,
          channel.clone(),
          delivery,
          pool.clone(),
          encryptor.clone(),
        )
            .await?;
      }
      // "test" => {
      //   // get type in message and call the appropriate handler
      //   let command_type = serde_json::from_str::<serde_json::Value>(message).unwrap();

      //   if command_type["type"].as_str().unwrap() == "fetch_once" {
      //     let message = serde_json::from_str::<MessageFetchOnce>(message).unwrap();
      //     // call fetch_once handler
      //     fetch_once(message, channel.clone(), delivery)
      //       .await
      //       .unwrap();
      //   };
      // }
      _ => (),
    };
  }

  Ok(())
}
