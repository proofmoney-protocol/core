use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;

pub struct KeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

pub fn generate_keypair() -> Result<KeyPair> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    Ok(KeyPair {
        signing_key,
        verifying_key,
    })
}
