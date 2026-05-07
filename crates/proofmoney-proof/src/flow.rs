use anyhow::{anyhow, Result};
use ed25519_dalek::VerifyingKey;
use serde_json::json;

use proofmoney_crypto::{is_valid_address, public_key_to_address, verify_signature};
use proofmoney_ledger::{
    has_sufficient_balance, transfer_signing_message, verify_event_hash,
};
use proofmoney_types::{Amount, Event, EventType, LedgerState, ProofResult};

pub fn verify_flow(transfer_event: &Event, state_before: &LedgerState) -> Result<ProofResult> {
    if transfer_event.event_type != EventType::Transfer {
        return Ok(ProofResult::invalid(
            "proof_of_flow",
            transfer_event.rule_version.clone(),
            json!({
                "event_id": transfer_event.id,
                "event_type": "not_transfer"
            }),
            "Flow proof is invalid. Event is not a transfer event.",
        ));
    }

    let from = transfer_event
        .payload
        .get("from")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let to = transfer_event
        .payload
        .get("to")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let amount = Amount::from_proof(
        transfer_event
            .payload
            .get("amount_proof")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
    );

    let fee = Amount::from_proof(
        transfer_event
            .payload
            .get("fee_proof")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
    );

    let signature = transfer_event
        .payload
        .get("signature")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let public_key_hex = transfer_event
        .payload
        .get("public_key")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let from_valid = is_valid_address(from, "tprm");
    let to_valid = is_valid_address(to, "tprm");
    let amount_positive = amount.0 > 0;
    let signature_present = !signature.is_empty();
    let hash_valid = verify_event_hash(transfer_event)?;

    let public_key_bytes = hex::decode(public_key_hex).unwrap_or_default();

    let expected_address = if public_key_bytes.len() == 32 {
        public_key_to_address(&public_key_bytes, "tprm").ok()
    } else {
        None
    };

    let address_matches_key = expected_address.as_deref() == Some(from);

    let signing_message = transfer_signing_message(from, to, amount, fee, &transfer_event.rule_version);

    let signature_valid = verify_with_public_key(&public_key_bytes, signing_message.as_bytes(), signature);

    let required = amount
        .checked_add(fee)
        .ok_or_else(|| anyhow!("transfer required amount overflow"))?;

    let balance_sufficient = has_sufficient_balance(state_before, from, required)?;

    let valid = from_valid
        && to_valid
        && amount_positive
        && signature_present
        && signature_valid
        && address_matches_key
        && balance_sufficient
        && hash_valid;

    let data = json!({
        "transaction_id": transfer_event.id,
        "from": from,
        "to": to,
        "amount": amount.to_prm_string(),
        "fee": fee.to_prm_string(),
        "from_address_valid": from_valid,
        "to_address_valid": to_valid,
        "amount_positive": amount_positive,
        "signature_present": signature_present,
        "signature_valid": signature_valid,
        "address_matches_public_key": address_matches_key,
        "sender_balance_sufficient": balance_sufficient,
        "event_hash_valid": hash_valid,
        "mvp_scope": "local_flow_validation"
    });

    if valid {
        Ok(ProofResult::valid(
            "proof_of_flow",
            transfer_event.rule_version.clone(),
            data,
            "Flow proof is valid within local MVP scope.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "proof_of_flow",
            transfer_event.rule_version.clone(),
            data,
            "Flow proof is invalid.",
        ))
    }
}

fn verify_with_public_key(public_key_bytes: &[u8], message: &[u8], signature_hex: &str) -> bool {
    let Ok(public_key_array) = <[u8; 32]>::try_from(public_key_bytes) else {
        return false;
    };

    let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key_array) else {
        return false;
    };

    verify_signature(&verifying_key, message, signature_hex)
}
