use anyhow::Result;
use serde_json::json;

use proofmoney_types::{LedgerState, ProofResult, RuleSet};

pub fn verify_supply(state: &LedgerState, rules: &RuleSet) -> Result<ProofResult> {
    let current_supply = state.current_supply;
    let valid = current_supply.0 <= rules.max_supply_boundary.0;

    let data = json!({
        "current_supply": current_supply.to_prm_string(),
        "maximum_supply_boundary": rules.max_supply_boundary.to_prm_string(),
        "initial_supply": rules.initial_supply.to_prm_string(),
        "public_proof_fund_total": state.public_fund_balance.to_prm_string(),
        "invalid_release_detected": !valid
    });

    if valid {
        Ok(ProofResult::valid(
            "proof_of_supply",
            rules.version.clone(),
            data,
            "Supply is valid. No supply boundary violation was detected.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "proof_of_supply",
            rules.version.clone(),
            data,
            "Supply is invalid. Supply boundary was exceeded.",
        ))
    }
}
