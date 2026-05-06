use anyhow::Result;
use serde_json::json;

use proofmoney_types::{LedgerState, ProofResult, ProofStatus, RuleSet};

use crate::{verify_rule, verify_supply};

pub fn integrity_status(state: &LedgerState, rules: &RuleSet) -> Result<ProofResult> {
    let supply = verify_supply(state, rules)?;
    let rule = verify_rule(rules, &RuleSet::default_v1())?;

    let healthy = matches!(supply.status, ProofStatus::Valid)
        && matches!(rule.status, ProofStatus::Valid);

    let data = json!({
        "starting_state": "available",
        "issuance": "available",
        "supply": format!("{:?}", supply.status),
        "ownership_verification": "available",
        "flow_verification": "available",
        "rule": format!("{:?}", rule.status),
        "public_proof_fund": "auditable",
        "invalid_release_detected": false,
        "mvp_stage": "local prototype"
    });

    if healthy {
        Ok(ProofResult::valid(
            "integrity_status",
            rules.version.clone(),
            data,
            "ProofMoney Integrity Status is healthy.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "integrity_status",
            rules.version.clone(),
            data,
            "ProofMoney Integrity Status is unhealthy.",
        ))
    }
}
