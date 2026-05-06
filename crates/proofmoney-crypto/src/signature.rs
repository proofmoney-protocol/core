use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> String {
    let signature: Signature = signing_key.sign(message);
    hex::encode(signature.to_bytes())
}

pub fn verify_signature(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature_hex: &str,
) -> bool {
    let Ok(bytes) = hex::decode(signature_hex) else {
        return false;
    };

    let Ok(signature) = Signature::from_slice(&bytes) else {
        return false;
    };

    verifying_key.verify(message, &signature).is_ok()
}
