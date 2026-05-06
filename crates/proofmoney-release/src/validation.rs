use anyhow::{bail, Result};
use serde_json::json;

use proofmoney_types::{Event, EventType, LedgerState, ProofResult, RuleSet};

use crate::{
    calculate_actual_release,
    calculate_contributor_reward,
    calculate_public_fund_allocation,
};

pub fn validate_release_event(
    event: &Event,
    state: &LedgerState,
    rules: &RuleSet,
) -> Result<ProofResult> {
    if event.event_type != EventType::Release {
        bail!("event is not a release event");
    }

    if event.rule_version != rules.version {
        bail!("release event rule version mismatch");
    }

    let interval = event
        .payload
        .get("interval")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow::anyhow!("missing interval"))?;

    let expected_total = calculate_actual_release(interval, rules);
    let expected_fund = calculate_public_fund_allocation(expected_total, rules);
    let expected_contributor = calculate_contributor_reward(expected_total, rules);

    let actual_total = event
        .payload
        .get("actual_release_proof")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow::anyhow!("missing actual release"))?;

    let actual_fund = event
        .payload
        .get("public_proof_fund_proof")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow::anyhow!("missing public fund allocation"))?;

    let actual_contributor = event
        .payload
        .get("proof_contributor_reward_proof")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow::anyhow!("missing contributor reward"))?;

    let privileged = event
        .payload
        .get("privileged_allocation")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let supply_after = state
        .current_supply
        .checked_add(expected_total)
        .ok_or_else(|| anyhow::anyhow!("supply overflow"))?;

    let valid = actual_total == expected_total.0
        && actual_fund == expected_fund.0
        && actual_contributor == expected_contributor.0
        && !privileged
        && supply_after.0 <= rules.max_supply_boundary.0;

    let data = json!({
        "event_id": event.id,
        "interval": interval,
        "actual_release": expected_total.to_prm_string(),
        "public_proof_fund": expected_fund.to_prm_string(),
        "proof_contributor_reward": expected_contributor.to_prm_string(),
        "privileged_allocation": privileged,
        "supply_boundary_exceeded": supply_after.0 > rules.max_supply_boundary.0
    });

    if valid {
        Ok(ProofResult::valid(
            "proof_of_issuance",
            rules.version.clone(),
            data,
            "Release event is valid under the active rule set.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "proof_of_issuance",
            rules.version.clone(),
            data,
            "Release event is invalid under the active rule set.",
        ))
    }
}
