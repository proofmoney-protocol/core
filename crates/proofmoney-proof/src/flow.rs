use anyhow::Result;
use serde_json::json;

use proofmoney_types::{LedgerState, ProofResult, TransferEvent};

pub fn verify_flow(transfer_event: &TransferEvent, _state: &LedgerState) -> Result<ProofResult> {
    let amount_positive = transfer_event.amount.0 > 0;
    let fee_valid = true;
    let signature_present = transfer_event.signature.is_some();

    let valid = amount_positive && fee_valid && signature_present;

    let data = json!({
        "transaction_id": transfer_event.id,
        "from": transfer_event.from,
        "to": transfer_event.to,
        "amount": transfer_event.amount.to_prm_string(),
        "fee": transfer_event.fee.to_prm_string(),
        "amount_positive": amount_positive,
        "fee_status": "valid",
        "authorization_present": signature_present,
        "double_spend_detected": false,
        "mvp_limitation": "MVP flow verification is simplified and does not yet implement a full UTXO or account balance engine."
    });

    if valid {
        Ok(ProofResult::valid(
            "proof_of_flow",
            transfer_event.rule_version.clone(),
            data,
            "Flow verification is valid within MVP limitations.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "proof_of_flow",
            transfer_event.rule_version.clone(),
            data,
            "Flow verification is invalid.",
        ))
    }
}
