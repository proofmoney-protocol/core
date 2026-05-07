use anyhow::{bail, Result};
use proofmoney_crypto::is_valid_address;
use proofmoney_ledger::{
    apply_event, create_transfer_event, state_before_event, transfer_signing_message,
};
use proofmoney_proof::verify_flow;
use proofmoney_storage::{default_wallet_path, load_json, load_or_init_ledger, save_ledger};
use proofmoney_types::{Amount, ProofStatus, RuleSet};
use proofmoney_wallet::{sign_message_with_private_key_hex, LocalWallet};

pub fn create_transfer(
    from: String,
    to: String,
    amount: String,
    append: bool,
    json: bool,
) -> Result<()> {
    if !is_valid_address(&from, "tprm") {
        bail!("invalid sender address");
    }

    if !is_valid_address(&to, "tprm") {
        bail!("invalid receiver address");
    }

    let amount = Amount::parse_prm_decimal(&amount)?;
    let fee = Amount::zero();

    if amount.is_zero() {
        bail!("amount must be greater than zero");
    }

    let wallet: LocalWallet = load_json(&default_wallet_path()?)?;

    if wallet.address != from {
        bail!("local wallet address does not match transfer sender");
    }

    let mut ledger = load_or_init_ledger("v1")?;
    let height = ledger.current_height + 1;

    let signing_message = transfer_signing_message(&from, &to, amount, fee, "v1");
    let signature = sign_message_with_private_key_hex(
        &wallet.private_key_hex_for_mvp_only,
        &signing_message,
    )?;

    let event = create_transfer_event(
        height,
        &from,
        &to,
        amount,
        fee,
        Some(signature),
        Some(wallet.public_key_hex.clone()),
        "v1",
    )?;

    let proof = verify_flow(&event, &ledger)?;

    if !matches!(proof.status, ProofStatus::Valid) {
        bail!("transfer failed Proof of Flow validation");
    }

    let mut append_status = "not_appended".to_string();

    if append {
        apply_event(&mut ledger, event.clone())?;
        save_ledger(&ledger)?;
        append_status = "appended".to_string();
    }

    let output = serde_json::json!({
        "transaction_id": event.id,
        "from": from,
        "to": to,
        "amount": amount.to_prm_string(),
        "fee": fee.to_prm_string(),
        "event_hash": event.hash,
        "append_status": append_status,
        "flow_status": format!("{:?}", proof.status),
        "mvp_scope": "local_transfer_event"
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Local Transfer Event\n");
        println!("Transaction ID: {}", event.id);
        println!("From: {}", from);
        println!("To: {}", to);
        println!("Amount: {}", amount.to_prm_string());
        println!("Fee: {}", fee.to_prm_string());
        println!("Event Hash: {}", event.hash);
        println!("Append Status: {}", append_status);
        println!("Flow Status: {:?}", proof.status);
        println!("Safety: Local MVP transfer only. No public network broadcast.");
    }

    Ok(())
}

pub fn verify_flow_by_tx(tx: String, json: bool) -> Result<()> {
    let ledger = load_or_init_ledger("v1")?;
    let (state_before, event) = state_before_event(&ledger, &tx)?;
    let proof = verify_flow(&event, &state_before)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!("Flow Verification\n");
        println!("{}", proof.summary);
        println!("Status: {:?}", proof.status);
    }

    Ok(())
}
