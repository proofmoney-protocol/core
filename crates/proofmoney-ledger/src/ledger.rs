use anyhow::{bail, Result};
use proofmoney_crypto::hash_bytes;
use proofmoney_types::{Amount, Event, EventType, LedgerState};

pub fn hash_ledger_state(state: &LedgerState) -> Result<String> {
    let encoded = serde_json::to_vec(state)?;
    Ok(hash_bytes(&encoded))
}

pub fn apply_event(state: &mut LedgerState, event: Event) -> Result<()> {
    if event.rule_version != state.rule_version {
        bail!("event rule version does not match ledger rule version");
    }

    if event.height <= state.current_height && !state.events.is_empty() {
        bail!("event height must be greater than current height");
    }

    if event.event_type == EventType::Release {
        if let Some(actual_release) = event.payload.get("actual_release_proof").and_then(|v| v.as_u64()) {
            state.current_supply = state
                .current_supply
                .checked_add(Amount::from_proof(actual_release))
                .ok_or_else(|| anyhow::anyhow!("supply overflow"))?;
        }

        if let Some(public_fund) = event.payload.get("public_proof_fund_proof").and_then(|v| v.as_u64()) {
            state.public_fund_balance = state
                .public_fund_balance
                .checked_add(Amount::from_proof(public_fund))
                .ok_or_else(|| anyhow::anyhow!("public fund overflow"))?;
        }
    }

    state.current_height = event.height;
    state.last_event_hash = Some(event.hash.clone());
    state.events.push(event);
    Ok(())
}
