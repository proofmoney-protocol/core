use anyhow::Result;
use serde_json::json;

use proofmoney_types::{ProofResult, RuleSet};

pub fn verify_rule(current: &RuleSet, expected: &RuleSet) -> Result<ProofResult> {
    let valid = current == expected;

    let data = json!({
        "active_rule_version": current.version,
        "maximum_supply_boundary": current.max_supply_boundary.to_prm_string(),
        "initial_supply": current.initial_supply.to_prm_string(),
        "public_fund_percent": current.public_fund_percent,
        "contributor_percent": current.contributor_percent,
        "unauthorized_rule_change": !valid
    });

    if valid {
        Ok(ProofResult::valid(
            "proof_of_rule",
            current.version.clone(),
            data,
            "Rule set is valid. No unauthorized rule change was detected.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "proof_of_rule",
            current.version.clone(),
            data,
            "Rule set is invalid. Unauthorized rule change was detected.",
        ))
    }
}
