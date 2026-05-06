use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use proofmoney_crypto::hash_bytes;
use proofmoney_types::{Amount, ProofResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartingState {
    pub initial_supply: Amount,
    pub founder_allocation: Amount,
    pub private_sale_allocation: Amount,
    pub hidden_address_allocation: Amount,
    pub public_proof_fund_preload: Amount,
    pub rule_version: String,
    pub state_hash: String,
}

impl StartingState {
    pub fn with_founder_allocation(amount: Amount) -> Result<Self> {
        let mut state = Self {
            initial_supply: Amount::zero(),
            founder_allocation: amount,
            private_sale_allocation: Amount::zero(),
            hidden_address_allocation: Amount::zero(),
            public_proof_fund_preload: Amount::zero(),
            rule_version: "v1".to_string(),
            state_hash: String::new(),
        };
        let encoded = serde_json::to_vec(&state)?;
        state.state_hash = hash_bytes(&encoded);
        Ok(state)
    }
}

pub fn generate_starting_state(rule_version: &str) -> Result<StartingState> {
    let mut state = StartingState {
        initial_supply: Amount::zero(),
        founder_allocation: Amount::zero(),
        private_sale_allocation: Amount::zero(),
        hidden_address_allocation: Amount::zero(),
        public_proof_fund_preload: Amount::zero(),
        rule_version: rule_version.to_string(),
        state_hash: String::new(),
    };

    let encoded = serde_json::to_vec(&state)?;
    state.state_hash = hash_bytes(&encoded);

    Ok(state)
}

pub fn verify_starting_state(state: &StartingState) -> Result<ProofResult> {
    let is_valid = state.initial_supply == Amount::zero()
        && state.founder_allocation == Amount::zero()
        && state.private_sale_allocation == Amount::zero()
        && state.hidden_address_allocation == Amount::zero()
        && state.public_proof_fund_preload == Amount::zero();

    let data = json!({
        "initial_supply": state.initial_supply.to_prm_string(),
        "founder_allocation": state.founder_allocation.to_prm_string(),
        "private_sale_allocation": state.private_sale_allocation.to_prm_string(),
        "hidden_address_allocation": state.hidden_address_allocation.to_prm_string(),
        "public_proof_fund_preload": state.public_proof_fund_preload.to_prm_string(),
        "state_hash": state.state_hash,
    });

    if is_valid {
        Ok(ProofResult::valid(
            "starting_state",
            state.rule_version.clone(),
            data,
            "Starting State is valid. No privileged allocation was detected.",
        ))
    } else {
        Ok(ProofResult::invalid(
            "starting_state",
            state.rule_version.clone(),
            data,
            "Starting State is invalid. Non-zero privileged allocation was detected.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proofmoney_types::ProofStatus;

    #[test]
    fn zero_starting_state_is_valid() {
        let state = generate_starting_state("v1").unwrap();
        let proof = verify_starting_state(&state).unwrap();
        assert_eq!(proof.status, ProofStatus::Valid);
    }

    #[test]
    fn non_zero_founder_allocation_is_invalid() {
        let state = StartingState::with_founder_allocation(Amount::from_prm(1)).unwrap();
        let proof = verify_starting_state(&state).unwrap();
        assert_eq!(proof.status, ProofStatus::Invalid);
    }
}
