use anyhow::Result;
use proofmoney_proof::verify_ownership as verify_ownership_proof;
use proofmoney_wallet::{create_wallet as create_local_wallet, sign_message_with_private_key_hex};

pub fn create_wallet(json: bool) -> Result<()> {
    let wallet = create_local_wallet()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&wallet)?);
    } else {
        println!("Experimental Wallet Created\n");
        println!("Wallet ID: {}", wallet.wallet_id);
        println!("Address: {}", wallet.address);
        println!("Public Key: {}", wallet.public_key_hex);
        println!("MVP Private Key: {}", wallet.private_key_hex_for_mvp_only);
        println!();
        println!("Warning: {}", wallet.private_key_warning);
        println!("This is experimental software. Do not use with valuable assets.");
    }

    Ok(())
}

pub fn sign_message(private_key: String, message: String, json: bool) -> Result<()> {
    let signature = sign_message_with_private_key_hex(&private_key, &message)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "message": message,
            "signature": signature,
            "warning": "MVP signing only. Do not use with valuable assets."
        }))?);
    } else {
        println!("Message Signature\n");
        println!("Message: {}", message);
        println!("Signature: {}", signature);
        println!("Warning: MVP signing only. Do not use with valuable assets.");
    }

    Ok(())
}

pub fn verify_ownership(
    address: String,
    message: String,
    signature: String,
    public_key: String,
    json: bool,
) -> Result<()> {
    let proof = verify_ownership_proof(&address, &message, &signature, &public_key)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!("Ownership Verification\n");
        println!("{}", proof.summary);
        println!("Status: {:?}", proof.status);
    }

    Ok(())
}
