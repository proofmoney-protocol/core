use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use proofmoney_crypto::{generate_keypair, public_key_to_address};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalWallet {
    pub wallet_id: String,
    pub address: String,
    pub public_key_hex: String,
    pub private_key_hex_for_mvp_only: String,
    pub private_key_warning: String,
    pub created_at: i64,
}

pub fn create_wallet() -> Result<LocalWallet> {
    let keypair = generate_keypair()?;
    let public_key_bytes = keypair.verifying_key.to_bytes();
    let signing_key_bytes = keypair.signing_key.to_bytes();

    let address = public_key_to_address(&public_key_bytes, "tprm")?;

    Ok(LocalWallet {
        wallet_id: Uuid::new_v4().to_string(),
        address,
        public_key_hex: hex::encode(public_key_bytes),
        private_key_hex_for_mvp_only: hex::encode(signing_key_bytes),
        private_key_warning: "MVP scaffold stores private key material only for local testing. Do not use with valuable assets."
            .to_string(),
        created_at: Utc::now().timestamp(),
    })
}
