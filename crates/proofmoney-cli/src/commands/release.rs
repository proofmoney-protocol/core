use anyhow::Result;
use proofmoney_ledger::apply_event;
use proofmoney_release::{
    calculate_actual_release, calculate_contributor_reward, calculate_public_fund_allocation,
    create_release_event, get_protection_factor_bps, validate_release_event,
};
use proofmoney_storage::{load_or_init_ledger, save_ledger};
use proofmoney_types::RuleSet;

pub fn simulate_release(
    interval: u64,
    recipient: Option<String>,
    append: bool,
    json: bool,
) -> Result<()> {
    let rules = RuleSet::default_v1();
    let recipient = recipient.unwrap_or_else(|| "tprm1example".to_string());

    let actual = calculate_actual_release(interval, &rules);
    let contributor = calculate_contributor_reward(actual, &rules);
    let fund = calculate_public_fund_allocation(actual, &rules);
    let factor = get_protection_factor_bps(interval, &rules);
    let event = create_release_event(interval, &recipient, "v1", &rules)?;

    let mut append_status = "not_appended".to_string();
    let mut ledger_supply_after = None;
    let mut ledger_public_fund_after = None;

    if append {
        let mut ledger = load_or_init_ledger("v1")?;
        let proof = validate_release_event(&event, &ledger, &rules)?;

        if !matches!(proof.status, proofmoney_types::ProofStatus::Valid) {
            anyhow::bail!("release event failed validation");
        }

        apply_event(&mut ledger, event.clone())?;
        save_ledger(&ledger)?;

        append_status = "appended".to_string();
        ledger_supply_after = Some(ledger.current_supply.to_prm_string());
        ledger_public_fund_after = Some(ledger.public_fund_balance.to_prm_string());
    }

    let output = serde_json::json!({
        "interval": interval,
        "recipient": recipient,
        "rule_version": rules.version,
        "base_release": rules.initial_release_amount.to_prm_string(),
        "protection_factor_bps": factor,
        "actual_release": actual.to_prm_string(),
        "proof_contributor_reward": contributor.to_prm_string(),
        "public_proof_fund": fund.to_prm_string(),
        "event_id": event.id,
        "event_hash": event.hash,
        "append_status": append_status,
        "ledger_supply_after": ledger_supply_after,
        "ledger_public_fund_after": ledger_public_fund_after,
        "status": "valid"
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Release Event Simulation\n");
        println!("Interval: {}", interval);
        println!("Recipient: {}", recipient);
        println!("Rule Version: {}", rules.version);
        println!("Base Release: {}", rules.initial_release_amount.to_prm_string());
        println!("Protection Factor BPS: {}", factor);
        println!("Actual Release: {}", actual.to_prm_string());
        println!("Proof Contributor Reward: {}", contributor.to_prm_string());
        println!("Public Proof Fund Allocation: {}", fund.to_prm_string());
        println!("Event ID: {}", event.id);
        println!("Event Hash: {}", event.hash);
        println!("Append Status: {}", append_status);

        if let Some(supply) = ledger_supply_after {
            println!("Ledger Supply After: {}", supply);
        }

        if let Some(public_fund) = ledger_public_fund_after {
            println!("Public Proof Fund After: {}", public_fund);
        }

        println!("Status: Valid");
    }

    Ok(())
}
