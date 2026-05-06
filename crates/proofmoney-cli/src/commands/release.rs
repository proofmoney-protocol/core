use anyhow::Result;
use proofmoney_release::{
    calculate_actual_release, calculate_contributor_reward, calculate_public_fund_allocation,
    create_release_event, get_protection_factor_bps,
};
use proofmoney_types::RuleSet;

pub fn simulate_release(interval: u64, json: bool) -> Result<()> {
    let rules = RuleSet::default_v1();

    let actual = calculate_actual_release(interval, &rules);
    let contributor = calculate_contributor_reward(actual, &rules);
    let fund = calculate_public_fund_allocation(actual, &rules);
    let factor = get_protection_factor_bps(interval, &rules);
    let event = create_release_event(interval, "tprm1example", "v1", &rules)?;

    let output = serde_json::json!({
        "interval": interval,
        "rule_version": rules.version,
        "base_release": rules.initial_release_amount.to_prm_string(),
        "protection_factor_bps": factor,
        "actual_release": actual.to_prm_string(),
        "proof_contributor_reward": contributor.to_prm_string(),
        "public_proof_fund": fund.to_prm_string(),
        "event_id": event.id,
        "event_hash": event.hash,
        "status": "valid"
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Release Event Simulation\n");
        println!("Interval: {}", interval);
        println!("Rule Version: {}", rules.version);
        println!("Base Release: {}", rules.initial_release_amount.to_prm_string());
        println!("Protection Factor BPS: {}", factor);
        println!("Actual Release: {}", actual.to_prm_string());
        println!("Proof Contributor Reward: {}", contributor.to_prm_string());
        println!("Public Proof Fund Allocation: {}", fund.to_prm_string());
        println!("Event ID: {}", event.id);
        println!("Event Hash: {}", event.hash);
        println!("Status: Valid");
    }

    Ok(())
}
