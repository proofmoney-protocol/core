use anyhow::Result;
use chrono::Utc;
use proofmoney_ledger::{apply_event, hash_event};
use proofmoney_storage::{load_or_init_ledger, save_ledger};
use proofmoney_types::{Amount, Event, EventType};
use serde_json::json;
use uuid::Uuid;

const SAFETY_NOTICE: &str = "Local MVP release simulation only. No public network exists and no PRM with monetary value is created.";

pub fn simulate_release(
    interval: u64,
    recipient: Option<String>,
    append: bool,
    json_output: bool,
) -> Result<()> {
    let mut ledger = load_or_init_ledger("v1")?;
    let recipient = recipient.unwrap_or_else(|| "tprm1proofcontributor".to_string());

    let actual_release = simulated_release_amount(interval);
    let public_fund = Amount::from_proof(actual_release.0 / 100 * 3);
    let contributor_reward = Amount::from_proof(actual_release.0 - public_fund.0);

    let mut event = Event {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::Release,
        height: ledger.current_height + 1,
        timestamp: Utc::now().timestamp(),
        rule_version: ledger.rule_version.clone(),
        payload: json!({
            "interval": interval,
            "recipient": recipient,
            "actual_release": actual_release.to_prm_string(),
            "public_proof_fund": public_fund.to_prm_string(),
            "proof_contributor_reward": contributor_reward.to_prm_string(),
            "actual_release_proof": actual_release.0,
            "public_proof_fund_proof": public_fund.0,
            "proof_contributor_reward_proof": contributor_reward.0,
            "mvp_scope": "local_release_simulation"
        }),
        hash: String::new(),
    };

    event.hash = hash_event(&event)?;

    let mut append_status = "not_appended".to_string();

    if append {
        apply_event(&mut ledger, event.clone())?;
        save_ledger(&ledger)?;
        append_status = "appended".to_string();
    }

    let output = json!({
        "event_id": event.id,
        "event_type": "Release",
        "height": event.height,
        "recipient": event.payload["recipient"],
        "interval": interval,
        "actual_release": actual_release.to_prm_string(),
        "public_proof_fund": public_fund.to_prm_string(),
        "proof_contributor_reward": contributor_reward.to_prm_string(),
        "event_hash": event.hash,
        "append_status": append_status,
        "safety_notice": SAFETY_NOTICE
    });

    if json_output {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("ProofMoney Local Release Simulation\n");
        println!("Interval: {}", interval);
        println!("Recipient: {}", output["recipient"].as_str().unwrap_or(""));
        println!("Actual Release: {}", actual_release.to_prm_string());
        println!("Public Proof Fund: {}", public_fund.to_prm_string());
        println!("Proof Contributor Reward: {}", contributor_reward.to_prm_string());
        println!("Event Hash: {}", output["event_hash"].as_str().unwrap_or(""));
        println!("Append Status: {}", append_status);
        println!("Safety: {}", SAFETY_NOTICE);
    }

    Ok(())
}

fn simulated_release_amount(interval: u64) -> Amount {
    let base = 5u64.saturating_mul(100_000_000);
    let interval_adjustment = interval.saturating_sub(1).saturating_mul(100_000);
    let proof_amount = base.saturating_sub(interval_adjustment).max(1);

    Amount::from_proof(proof_amount)
}
