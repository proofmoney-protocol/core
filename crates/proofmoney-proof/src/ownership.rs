use anyhow::{anyhow, Result};
use ed25519_dalek::VerifyingKey;
use serde_json::json;

use proofmoney_crypto::{is_valid_address, public_key_to_address, verify_signature};
use proofmoney_types::ProofResult;

pub fn verify_ownership(
    address: &str,
    message: &str,
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<ProofResult> {
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| anyhow!("invalid public key hex: {e}"))?;
    let verifying_key = VerifyingKey::from_bytes(
        public_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("public key must be 32 bytes"))?,
    )
    .map_err(|e| anyhow!("invalid verifying key: {e}"))?;

    let expected_address = public_key_to_address(&public_key_bytes, "tprm")?;
    let address_valid = is_valid_address(address, "tprm");
    let address_matches_key = expected_address == address;
    let signature_valid = verify_signature(&verifying_key, message.as_bytes(), signature_hex);

    let valid = address_valid && address_matches_key && signature_valid;

    let data = json!({
        "address": address,
        "expected_address": expected_address,
        "control_type": "self-controlled",
        "address_valid": address_valid,
        "address_matches_key": address_matches_key,
        "signature_valid": signature_valid,
        "custodial_risk": "not_detected"
    });

    if valid {
        Ok(ProofResult::valid(
            "proof_of_ownership",
            "v1",
            data,
            "Ownership proof is valid. The address and signature were verified.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "proof_of_ownership",
            "v1",
            data,
            "Ownership proof is invalid. Address or signature verification failed.",
        ))
    }
}
