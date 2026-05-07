use chrono::Utc;
use proofmoney_ledger::{apply_event, create_transfer_event, hash_event, verify_event_hash};
use proofmoney_types::{Amount, Event, EventType, LedgerState};
use serde_json::json;
use uuid::Uuid;

fn sample_release_event(rule_version: &str, height: u64) -> Event {
    let mut event = Event {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::Release,
        height,
        timestamp: Utc::now().timestamp(),
        rule_version: rule_version.to_string(),
        payload: json!({
            "recipient": "tprm1internalreview",
            "actual_release_proof": 500_000_000u64,
            "proof_contributor_reward_proof": 485_000_000u64,
            "public_proof_fund_proof": 15_000_000u64
        }),
        hash: String::new(),
    };

    event.hash = hash_event(&event).expect("release event should hash");
    event
}

#[test]
fn invalid_event_hash_is_rejected() {
    let mut state = LedgerState::new("v1");
    let mut event = sample_release_event("v1", 1);

    event.hash = "tampered-hash".to_string();

    let result = apply_event(&mut state, event);
    assert!(result.is_err());
}

#[test]
fn rule_version_mismatch_is_rejected() {
    let mut state = LedgerState::new("v1");
    let event = sample_release_event("v2", 1);

    let result = apply_event(&mut state, event);
    assert!(result.is_err());
}

#[test]
fn insufficient_balance_transfer_is_rejected() {
    let mut state = LedgerState::new("v1");

    let event = create_transfer_event(
        1,
        "tprm1sender",
        "tprm1receiver",
        Amount::from_prm(1),
        Amount::zero(),
        Some("sample-signature".to_string()),
        Some("sample-public-key".to_string()),
        "v1",
    )
    .expect("transfer event should be created");

    let result = apply_event(&mut state, event);
    assert!(result.is_err());
}

#[test]
fn mutated_transfer_event_hash_is_detected() {
    let mut event = create_transfer_event(
        1,
        "tprm1sender",
        "tprm1receiver",
        Amount::from_prm(1),
        Amount::zero(),
        Some("sample-signature".to_string()),
        Some("sample-public-key".to_string()),
        "v1",
    )
    .expect("transfer event should be created");

    assert!(verify_event_hash(&event).expect("hash verification should run"));

    event.payload["amount_proof"] = json!(200_000_000u64);

    assert!(!verify_event_hash(&event).expect("hash verification should run"));
}
