use std::sync::Arc;

use lapin::{auth::Credentials, Channel, Connection, ConnectionProperties, ExchangeKind, Queue};
use lapin::options::{ExchangeDeclareOptions, QueueDeclareOptions};
use lapin::types::{AMQPValue, FieldTable};

pub struct LapinClient {
    pub conn: Connection,
    pub channel: Arc<Channel>,
}

impl LapinClient {
    pub async fn new(host: String, port: u16, creds: Credentials) -> lapin::Result<Self> {
        let uri = format!(
            "amqp://{}:{}@{}:{}/%2f",
            creds.username(),
            creds.password(),
            host,
            port
        );

        println!("{}", uri);

        let conn = Connection::connect(&uri, ConnectionProperties::default()).await?;

        let channel = conn.create_channel().await?;
        let channel = Arc::new(channel);

        Ok(Self { conn, channel })
    }

    pub async fn close(&self) {
        self.conn.close(200, "Goodbye").await.unwrap();
    }

    pub async fn declare_exchange(&self, name: &str, kind: ExchangeKind) -> lapin::Result<()> {
        self
            .channel
            .exchange_declare(
                name,
                kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
    }

    pub async fn declare_queue(&self, name: &str, durable: bool) -> lapin::Result<Queue> {
        let mut arguments = FieldTable::default();

        arguments.insert(
            "x-queue-type".into(),
            AMQPValue::LongString("quorum".into()),
        );

        self
            .channel
            .queue_declare(
                name,
                QueueDeclareOptions {
                    durable,
                    ..Default::default()
                },
                arguments,
            )
            .await
    }

    pub async fn configure_service(&self) -> Result<(), Box<dyn std::error::Error>> {
        let exchange_name = "glycoflow_topic_exchange";

        self
            .declare_exchange(&exchange_name, ExchangeKind::Topic)
            .await?;

        self
            .declare_queue("glycoflow_register_access_token", true)
            .await?;
        self.declare_queue("glycoflow_fetch_once", true).await?;
        self.declare_queue("glycoflow_register_data", true).await?;

        self
            .channel
            .queue_bind(
                "glycoflow_register_access_token",
                &exchange_name,
                "glycoflow.register.access_token.v1",
                Default::default(),
                Default::default(),
            )
            .await?;

        self
            .channel
            .queue_bind(
                "glycoflow_fetch_once",
                &exchange_name,
                "glycoflow.fetch.once.v1",
                Default::default(),
                Default::default(),
            )
            .await?;

        self
            .channel
            .queue_bind(
                "glycoflow_register_data",
                &exchange_name,
                "glycoflow.register.data.v1",
                Default::default(),
                Default::default(),
            )
            .await?;

        Ok(())
    }
}
