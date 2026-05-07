use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use proofmoney_crypto::hash_bytes;
use proofmoney_types::{Amount, Event, EventType, LedgerState};
use serde_json::json;
use std::collections::BTreeMap;
use uuid::Uuid;

pub const PUBLIC_PROOF_FUND_ACCOUNT: &str = "public_proof_fund";

pub fn hash_ledger_state(state: &LedgerState) -> Result<String> {
    let encoded = serde_json::to_vec(state)?;
    Ok(hash_bytes(&encoded))
}

pub fn hash_event(event: &Event) -> Result<String> {
    let encoded = serde_json::to_vec(&event.hash_view())?;
    Ok(hash_bytes(&encoded))
}

pub fn verify_event_hash(event: &Event) -> Result<bool> {
    Ok(hash_event(event)? == event.hash)
}

pub fn transfer_signing_message(
    from: &str,
    to: &str,
    amount: Amount,
    fee: Amount,
    rule_version: &str,
) -> String {
    format!(
        "proofmoney-transfer:{}:{}:{}:{}:{}",
        rule_version, from, to, amount.0, fee.0
    )
}

pub fn create_transfer_event(
    height: u64,
    from: &str,
    to: &str,
    amount: Amount,
    fee: Amount,
    signature: Option<String>,
    public_key: Option<String>,
    rule_version: &str,
) -> Result<Event> {
    let signing_message = transfer_signing_message(from, to, amount, fee, rule_version);

    let payload = json!({
        "from": from,
        "to": to,
        "amount": amount.to_prm_string(),
        "amount_proof": amount.0,
        "fee": fee.to_prm_string(),
        "fee_proof": fee.0,
        "signature": signature,
        "public_key": public_key,
        "signing_message": signing_message,
    });

    let mut event = Event {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::Transfer,
        height,
        timestamp: Utc::now().timestamp(),
        rule_version: rule_version.to_string(),
        payload,
        hash: String::new(),
    };

    event.hash = hash_event(&event)?;
    Ok(event)
}

pub fn compute_supply_from_events(state: &LedgerState) -> Result<Amount> {
    let mut supply = Amount::zero();

    for event in &state.events {
        if event.event_type == EventType::Release {
            let release = event
                .payload
                .get("actual_release_proof")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("release event missing actual_release_proof"))?;

            supply = supply
                .checked_add(Amount::from_proof(release))
                .ok_or_else(|| anyhow!("computed supply overflow"))?;
        }
    }

    Ok(supply)
}

pub fn compute_public_fund_from_events(state: &LedgerState) -> Result<Amount> {
    let balances = compute_balances_from_events(state)?;
    Ok(*balances
        .get(PUBLIC_PROOF_FUND_ACCOUNT)
        .unwrap_or(&Amount::zero()))
}

pub fn compute_balances_from_events(state: &LedgerState) -> Result<BTreeMap<String, Amount>> {
    let mut balances: BTreeMap<String, Amount> = BTreeMap::new();

    for event in &state.events {
        match event.event_type {
            EventType::Release => {
                let recipient = event
                    .payload
                    .get("recipient")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("release event missing recipient"))?
                    .to_string();

                let contributor_reward = event
                    .payload
                    .get("proof_contributor_reward_proof")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| anyhow!("release event missing contributor reward"))?;

                let public_fund = event
                    .payload
                    .get("public_proof_fund_proof")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| anyhow!("release event missing public fund"))?;

                credit(
                    &mut balances,
                    &recipient,
                    Amount::from_proof(contributor_reward),
                )?;
                credit(
                    &mut balances,
                    PUBLIC_PROOF_FUND_ACCOUNT,
                    Amount::from_proof(public_fund),
                )?;
            }
            EventType::Transfer => {
                let from = event
                    .payload
                    .get("from")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("transfer event missing from"))?
                    .to_string();

                let to = event
                    .payload
                    .get("to")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("transfer event missing to"))?
                    .to_string();

                let amount = Amount::from_proof(
                    event
                        .payload
                        .get("amount_proof")
                        .and_then(|v| v.as_u64())
                        .ok_or_else(|| anyhow!("transfer event missing amount"))?,
                );

                let fee = Amount::from_proof(
                    event
                        .payload
                        .get("fee_proof")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0),
                );

                let required = amount
                    .checked_add(fee)
                    .ok_or_else(|| anyhow!("transfer required amount overflow"))?;

                debit(&mut balances, &from, required)?;
                credit(&mut balances, &to, amount)?;
                credit(&mut balances, PUBLIC_PROOF_FUND_ACCOUNT, fee)?;
            }
            _ => {}
        }
    }

    Ok(balances)
}

pub fn get_balance(state: &LedgerState, address: &str) -> Result<Amount> {
    let balances = compute_balances_from_events(state)?;
    Ok(*balances.get(address).unwrap_or(&Amount::zero()))
}

pub fn has_sufficient_balance(state: &LedgerState, address: &str, required: Amount) -> Result<bool> {
    Ok(get_balance(state, address)?.0 >= required.0)
}

fn credit(balances: &mut BTreeMap<String, Amount>, address: &str, amount: Amount) -> Result<()> {
    let current = *balances.get(address).unwrap_or(&Amount::zero());
    let updated = current
        .checked_add(amount)
        .ok_or_else(|| anyhow!("balance overflow"))?;
    balances.insert(address.to_string(), updated);
    Ok(())
}

fn debit(balances: &mut BTreeMap<String, Amount>, address: &str, amount: Amount) -> Result<()> {
    let current = *balances.get(address).unwrap_or(&Amount::zero());

    if current.0 < amount.0 {
        bail!("insufficient balance");
    }

    let updated = current
        .checked_sub(amount)
        .ok_or_else(|| anyhow!("balance underflow"))?;
    balances.insert(address.to_string(), updated);
    Ok(())
}

pub fn apply_event(state: &mut LedgerState, event: Event) -> Result<()> {
    if event.rule_version != state.rule_version {
        bail!("event rule version does not match ledger rule version");
    }

    if !verify_event_hash(&event)? {
        bail!("event hash is invalid");
    }

    if event.height <= state.current_height && !state.events.is_empty() {
        bail!("event height must be greater than current height");
    }

    if event.event_type == EventType::Transfer {
        let from = event
            .payload
            .get("from")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("transfer event missing from"))?;

        let amount = Amount::from_proof(
            event
                .payload
                .get("amount_proof")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| anyhow!("transfer event missing amount"))?,
        );

        let fee = Amount::from_proof(
            event
                .payload
                .get("fee_proof")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
        );

        let required = amount
            .checked_add(fee)
            .ok_or_else(|| anyhow!("transfer required amount overflow"))?;

        if !has_sufficient_balance(state, from, required)? {
            bail!("insufficient balance");
        }
    }

    state.events.push(event.clone());
    state.current_supply = compute_supply_from_events(state)?;
    state.public_fund_balance = compute_public_fund_from_events(state)?;
    state.current_height = event.height;
    state.last_event_hash = Some(event.hash);

    Ok(())
}

pub fn state_before_event(state: &LedgerState, event_id: &str) -> Result<(LedgerState, Event)> {
    let mut prior = LedgerState::new(&state.rule_version);

    for event in &state.events {
        if event.id == event_id {
            return Ok((prior, event.clone()));
        }

        apply_event(&mut prior, event.clone())?;
    }

    bail!("event not found")
}

pub fn reset_local_ledger(rule_version: &str) -> LedgerState {
    LedgerState::new(rule_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use proofmoney_types::RuleSet;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn event_hash_excludes_hash_field() {
        let mut event = Event {
            id: Uuid::new_v4().to_string(),
            event_type: EventType::Release,
            height: 1,
            timestamp: Utc::now().timestamp(),
            rule_version: "v1".to_string(),
            payload: json!({"actual_release_proof": 1, "public_proof_fund_proof": 0}),
            hash: String::new(),
        };

        let hash = hash_event(&event).unwrap();
        event.hash = hash.clone();

        assert_eq!(hash_event(&event).unwrap(), hash);
        assert!(verify_event_hash(&event).unwrap());

        event.hash = "bad".to_string();
        assert!(!verify_event_hash(&event).unwrap());
    }

    #[test]
    fn computed_supply_sums_release_events() {
        let rules = RuleSet::default_v1();
        let mut state = LedgerState::new(&rules.version);

        let mut event = Event {
            id: Uuid::new_v4().to_string(),
            event_type: EventType::Release,
            height: 1,
            timestamp: Utc::now().timestamp(),
            rule_version: "v1".to_string(),
            payload: json!({
                "recipient": "tprm1example",
                "actual_release_proof": 500_000_000u64,
                "proof_contributor_reward_proof": 485_000_000u64,
                "public_proof_fund_proof": 15_000_000u64
            }),
            hash: String::new(),
        };
        event.hash = hash_event(&event).unwrap();

        apply_event(&mut state, event).unwrap();

        assert_eq!(state.current_supply.to_prm_string(), "5.00000000");
        assert_eq!(state.public_fund_balance.to_prm_string(), "0.15000000");
        assert_eq!(get_balance(&state, "tprm1example").unwrap().to_prm_string(), "4.85000000");
    }

    #[test]
    fn insufficient_transfer_is_rejected() {
        let mut state = LedgerState::new("v1");
        let mut transfer = create_transfer_event(
            1,
            "tprm1sender",
            "tprm1receiver",
            Amount::from_prm(1),
            Amount::zero(),
            Some("signature".to_string()),
            Some("public_key".to_string()),
            "v1",
        )
        .unwrap();

        transfer.hash = hash_event(&transfer).unwrap();

        assert!(apply_event(&mut state, transfer).is_err());
    }
}
