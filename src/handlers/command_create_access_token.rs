use std::sync::Arc;

use lapin::{Channel, message::Delivery, options::BasicAckOptions};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};

use crate::encryptor::{EncryptionError, Encryptor};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageCreateAccessToken {
  user_id: String,
  access_token: String,
  patient_id: String,
}

pub async fn create_access_token(
  message: MessageCreateAccessToken,
  channel: Arc<Channel>,
  delivery: Delivery,
  pool: Arc<PgPool>,
  encryptor: Arc<Encryptor>,
) -> Result<(), Box<dyn std::error::Error>> {
  let (encrypted_token, nonce) = match encryptor.encrypt(&message.access_token) {
    Ok((enc_token, nonce)) => (enc_token, nonce),
    Err(err) => {
      error!(
        "Failed to encrypt access token for user_id: {}: {:?}",
        message.user_id, err
      );
      return Err(Box::new(EncryptionError(err)));
    }
  };

  let t = encryptor
      .encrypt_with_nonce(&message.patient_id, &nonce)
      .unwrap();

  match sqlx::query!(
    "INSERT INTO access_tokens (user_id, access_token, patient_id, nonce) VALUES ($1, $2, $3, $4)",
    message.user_id,
    encrypted_token,
    t,
    nonce
  )
      .execute(pool.as_ref())
      .await
  {
    Ok(_) => {
      info!(
        "Access token created successfully for user_id: {}",
        message.user_id
      );
    }
    Err(err) => {
      error!(
        "Failed to create access token for user_id: {}: {:?}",
        message.user_id, err
      );
      return Err(Box::new(err));
    }
  }

  // Acknowledge the message
  if let Err(err) = channel
      .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
      .await
  {
    error!("Failed to acknowledge message: {:?}", err);
    return Err(Box::new(err));
  }
  Ok(())
}
