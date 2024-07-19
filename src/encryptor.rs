use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use base64::{Engine, engine::general_purpose};

pub struct Encryptor {
    key: Key<Aes256Gcm>,
}

#[derive(Debug)]
pub struct EncryptionError(pub aes_gcm::Error);

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Encryption error: {}", self.0)
    }
}

impl std::error::Error for EncryptionError {}

impl Encryptor {
    pub fn new(key_str: String) -> Result<Encryptor, Box<dyn std::error::Error>> {
        let key_bytes = general_purpose::STANDARD.decode(key_str)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

        Ok(Encryptor { key: *key })
    }

    pub fn encrypt(&self, data: &str) -> Result<(Vec<u8>, Vec<u8>), aes_gcm::Error> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, data.as_ref())?;

        Ok((ciphertext, nonce.to_vec()))
    }

    pub fn encrypt_with_nonce(&self, data: &str, nonce: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(nonce);

        let encrypted_data = cipher.encrypt(nonce, data.as_ref())?;
        Ok(encrypted_data)
    }

    pub fn decrypt(&self, encrypted_data: &[u8], nonce: &[u8]) -> Result<String, aes_gcm::Error> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(nonce);

        let decrypted_data = cipher.decrypt(nonce, encrypted_data)?;

        let result = String::from_utf8(decrypted_data).map_err(|_| aes_gcm::Error)?;

        Ok(result)
    }
}
