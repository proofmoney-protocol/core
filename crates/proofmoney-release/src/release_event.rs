use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use proofmoney_crypto::hash_bytes;
use proofmoney_types::{Event, EventType, RuleSet};

use crate::{
    calculate_actual_release,
    calculate_contributor_reward,
    calculate_public_fund_allocation,
    get_protection_factor_bps,
};

pub fn create_release_event(
    interval: u64,
    recipient: &str,
    rule_version: &str,
    rules: &RuleSet,
) -> Result<Event> {
    let total_release = calculate_actual_release(interval, rules);
    let contributor_reward = calculate_contributor_reward(total_release, rules);
    let public_fund = calculate_public_fund_allocation(total_release, rules);

    let payload = json!({
        "interval": interval,
        "recipient": recipient,
        "base_release": rules.initial_release_amount.to_prm_string(),
        "protection_factor_bps": get_protection_factor_bps(interval, rules),
        "actual_release": total_release.to_prm_string(),
        "actual_release_proof": total_release.0,
        "proof_contributor_reward": contributor_reward.to_prm_string(),
        "proof_contributor_reward_proof": contributor_reward.0,
        "public_proof_fund": public_fund.to_prm_string(),
        "public_proof_fund_proof": public_fund.0,
        "privileged_allocation": false
    });

    let mut event = Event {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::Release,
        height: interval,
        timestamp: Utc::now().timestamp(),
        rule_version: rule_version.to_string(),
        payload,
        hash: String::new(),
    };

    let encoded = serde_json::to_vec(&event)?;
    event.hash = hash_bytes(&encoded);

    Ok(event)
}
