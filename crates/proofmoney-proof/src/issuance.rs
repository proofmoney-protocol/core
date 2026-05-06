use anyhow::Result;

use proofmoney_release::validate_release_event;
use proofmoney_types::{Event, LedgerState, ProofResult, RuleSet};

pub fn verify_issuance(
    event: &Event,
    state: &LedgerState,
    rules: &RuleSet,
) -> Result<ProofResult> {
    validate_release_event(event, state, rules)
}
