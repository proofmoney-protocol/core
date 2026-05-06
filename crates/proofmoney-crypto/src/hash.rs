use sha2::{Digest, Sha256};

pub fn hash_bytes(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    hex::encode(digest)
}

pub fn hash_json(value: &serde_json::Value) -> anyhow::Result<String> {
    let encoded = serde_json::to_vec(value)?;
    Ok(hash_bytes(&encoded))
}
