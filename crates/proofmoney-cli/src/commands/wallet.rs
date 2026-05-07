use anyhow::{bail, Result};
use proofmoney_proof::verify_ownership as verify_ownership_proof;
use proofmoney_storage::{default_wallet_path, load_json, save_json};
use proofmoney_wallet::{
    create_wallet as create_local_wallet, sign_message_with_private_key_hex, LocalWallet,
};

pub fn create_wallet(force: bool, json: bool) -> Result<()> {
    let path = default_wallet_path()?;

    if path.exists() && !force {
        bail!(
            "wallet already exists at {}. Use --force to overwrite in MVP testing.",
            path.display()
        );
    }

    let wallet = create_local_wallet()?;
    save_json(&path, &wallet)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "wallet": wallet,
                "wallet_path": path.display().to_string(),
                "warning": "MVP wallet only. Do not use with valuable assets."
            }))?
        );
    } else {
        println!("Experimental Wallet Created\n");
        println!("Wallet ID: {}", wallet.wallet_id);
        println!("Address: {}", wallet.address);
        println!("Public Key: {}", wallet.public_key_hex);
        println!("Wallet Path: {}", path.display());
        println!();
        println!("Warning: {}", wallet.private_key_warning);
        println!("This is experimental software. Do not use with valuable assets.");
    }

    Ok(())
}

pub fn new_address(json: bool) -> Result<()> {
    let path = default_wallet_path()?;
    let wallet: LocalWallet = load_json(&path)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "address": wallet.address,
                "public_key": wallet.public_key_hex,
                "wallet_path": path.display().to_string(),
                "prefix": "tprm"
            }))?
        );
    } else {
        println!("ProofMoney Local Address\n");
        println!("Address: {}", wallet.address);
        println!("Public Key: {}", wallet.public_key_hex);
        println!("Wallet Path: {}", path.display());
        println!("Prefix: tprm");
    }

    Ok(())
}

pub fn sign_message(private_key: Option<String>, message: String, json: bool) -> Result<()> {
    let (private_key, address, public_key, key_source) = if let Some(private_key) = private_key {
        (
            private_key,
            None,
            None,
            "explicit_private_key_for_mvp_testing".to_string(),
        )
    } else {
        let wallet: LocalWallet = load_json(&default_wallet_path()?)?;
        (
            wallet.private_key_hex_for_mvp_only,
            Some(wallet.address),
            Some(wallet.public_key_hex),
            "local_wallet".to_string(),
        )
    };

    let signature = sign_message_with_private_key_hex(&private_key, &message)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "message": message,
                "signature": signature,
                "address": address,
                "public_key": public_key,
                "key_source": key_source,
                "warning": "MVP signing only. Do not use with valuable assets."
            }))?
        );
    } else {
        println!("Message Signature\n");
        if let Some(address) = address {
            println!("Address: {}", address);
        }
        if let Some(public_key) = public_key {
            println!("Public Key: {}", public_key);
        }
        println!("Message: {}", message);
        println!("Signature: {}", signature);
        println!("Key Source: {}", key_source);
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
