use std::sync::Arc;

use lapin::{BasicProperties, Channel, message::Delivery};
use lapin::options::BasicPublishOptions;
use librelink_client::client::LibreLinkClient;
use librelink_client::connection::GraphData;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query};

use crate::encryptor::Encryptor;
use crate::messages::commands::FetchOnce;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFetchOnce {
    #[serde(rename = "type")]
    _type: String,
    payload: FetchOnce,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageSendFetchOnce {
    patient_id: String,
    data: Vec<GraphData>,
}

pub async fn fetch_once(
    message: MessageFetchOnce,
    channel: Arc<Channel>,
    _delivery: Delivery,
    pool: Arc<PgPool>,
    encryptor: Arc<Encryptor>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Received message: {:?}", message);
    let row = query!(
    "SELECT access_token, nonce, patient_id FROM access_tokens WHERE user_id = $1",
    message.payload.user_id
  )
        .fetch_one(pool.as_ref())
        .await?;

    let access_token: Vec<u8> = row.access_token;
    let nonce: Vec<u8> = row.nonce;
    let patient_id: Vec<u8> = row.patient_id;

    let access_token = encryptor.decrypt(&access_token, &nonce).unwrap();
    let patient_id = encryptor.decrypt(&patient_id, &nonce).unwrap();

    let libre_link = LibreLinkClient::from_token(access_token, Some("fr".to_string()));

    let t = libre_link.get_log_book(&patient_id).await?;

    let tt = MessageSendFetchOnce {
        data: t.data,
        patient_id,
    };

    channel
        .basic_publish(
            "glycoflow_topic_exchange",
            "glycoflow.register.data.v1",
            BasicPublishOptions::default(),
            &*serde_json::to_vec(&tt).unwrap(),
            BasicProperties::default(),
        ).await?;

    Ok(())
}
