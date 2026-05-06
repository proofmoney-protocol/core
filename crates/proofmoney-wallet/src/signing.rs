use anyhow::{anyhow, Result};
use ed25519_dalek::SigningKey;

use proofmoney_crypto::sign_message;

pub fn sign_message_with_private_key_hex(private_key_hex: &str, message: &str) -> Result<String> {
    let bytes = hex::decode(private_key_hex).map_err(|e| anyhow!("invalid private key hex: {e}"))?;
    let signing_key = SigningKey::from_bytes(
        bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("private key must be 32 bytes"))?,
    );

    Ok(sign_message(&signing_key, message.as_bytes()))
}
